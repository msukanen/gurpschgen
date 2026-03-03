//!
//! Basic GURPS DTA/GEN file converter to convert those files into format `gurpschgen` understands.
//! 
//! Copyright © 2024-2026 Markku Sukanen
//! 
//! DISCLAIMER: feel free to use the code as you see fit - be it for your own use,
//!             derivate work, commercial capacity or whatever else.
//! 
mod verify_dta;
mod combine_lines;
mod skill;
mod context;
mod categorypayload;
mod adq;
pub(crate) mod equipment;
mod ranged;
mod rof;
mod shots;
mod melee;
mod armor;
mod weapon;
mod damage;
mod item;
mod container;
mod stat;
mod difficultyrating;
mod skillroot;

use std::{fs, path::PathBuf, str::FromStr};

use clap::Parser;
use either::Either;
use glob::{MatchOptions, glob_with};
use gurpschgen_lib::dta::{locate_dta::locate_dta, read_lines::read_lines, genre::GENRE_MANIFEST_FN};
use once_cell::sync::Lazy;
use regex::Regex;
use verify_dta::verify_and_categorize_dta;

static RX_COST_WEIGHT: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:\s*(?<cost>\d+(?:[.]\d+)?)(?:\s*,\s*(?<wt>\d+(?:[.]\d+)?))?)").unwrap());
const MISSING_FILE_MARKER: &'static str = "<missing>";
const LEGACY_GENRE_DTA: &'static str = "GENRE.DTA";

// pub(crate) const KNOWN_OFFENDER_FILES: [&'static str; 2] = [
//     "GRIMOIRE.DTA",
//     "TEST3.DTA",
// ];

#[derive(Parser)]
struct Cli {
    /// Note[^][Cli]
    #[arg(conflicts_with = "auto")]
    path: Option<PathBuf>,
    
    /// Note[^][Cli]
    #[arg(long)]
    test: bool,
    
    /// Note[^][Cli]
    #[arg(long, conflicts_with = "path")]
    auto: bool,

    #[arg(short,long)]
    verbose: bool,
    
    #[arg(long, value_delimiter = ',')]
    skip: Vec<String>,
}

/// Swift to-percentage conversion for "total minus x (of total)".
const fn to_percentage(x_of: f64, total: f64) -> f64 {
    ((total - x_of) / total) * 100.0
}

/// The main culprit for all the pain and suffering… ;-)
fn main() {
    let args = Cli::parse();
    let _ = env_logger::try_init();

    // pinpoint where DTAs live and move there
    locate_dta(args.verbose);
    
    if !args.auto && args.path.is_none() {
        println!("You need to either specify name of a datafile to process, or use --batch flag if you want to crunch them all at once…");
        return;
    }
    
    if args.auto {
        auto_generate_manifest_and_data(&args);
    } else {
        // deal with on cmdline defined path.
        let path = args.path.unwrap();
        
        log::info!("Processing {}", path.display());

        // final pretty print as JSON
        println!("{}", match verify_and_categorize_dta(&path, read_lines(path.clone()), args.verbose) {
            Either::Left(dtalib) => serde_json::to_string_pretty(&dtalib).unwrap(),
            Either::Right(mfpack) => serde_json::to_string_pretty(&mfpack).unwrap()
        })
    }
}

const fn maybe_test_prefix(in_test_mode: bool) -> &'static str {
    match in_test_mode {
        true => "test-",
        _ => ""
    }
}

const fn maybe_plural_s(num: usize) -> &'static str {
    match num {
        1 => "",
        _ => "s"
    }
}

fn mk_gch_manifest_filename(test: bool) -> String {
    format!("{}{GENRE_MANIFEST_FN}", maybe_test_prefix(test))
}

/// Auto-generate genre manifest and all the associated sidecar data files.
fn auto_generate_manifest_and_data(args: &Cli) {
    // first we deal with GENRE.DTA, if such is present, but if no such is found we bail out.
    let mut mfpack = verify_and_categorize_dta(
            &PathBuf::from_str(LEGACY_GENRE_DTA).unwrap(),
            read_lines(LEGACY_GENRE_DTA),
            args.verbose
        ).expect_right(format!("We didn't manage to deal with {LEGACY_GENRE_DTA}…").as_str());
    
    // Genre by genre…
    for mf in mfpack.genres.iter_mut() {
        let mut misses = 0;
        let mut approx = 0;
        // Let's deal with each individual DTA file mentioned…
        for req_fn in mf.files.iter_mut() {
            let mut dta_fn = PathBuf::from_str(&req_fn).unwrap();
            // If no 1:1 matching DTA file exists (due a typo, too long file name, etc.),
            // see if we find *something* that matches "close enough":
            if !dta_fn.exists() {
                let stem = dta_fn.file_stem().unwrap().to_str().unwrap();
                // The true legacy files have MS-DOS era 8.3 names. We do *not* attempt to deal with prehistoric Windows' 8.3 mangling...
                if stem.len() > 8 {
                    let trunc = format!("{}.dta", &stem[..8]);
                    dta_fn = glob_with(trunc.as_str(), MatchOptions { case_sensitive: false, ..MatchOptions::default() }).unwrap()
                        .into_iter().next()
                        .unwrap().unwrap();
                    if !dta_fn.exists() {
                        log::error!("No such file (or reasonable variant there of) present as '{}'", dta_fn.display());
                        continue;
                    }

                    approx += 1;
                    *req_fn = dta_fn.file_name().unwrap().to_str().unwrap().into();// crossing fingers here… X-D
                    log::warn!("'{}' not found. Using enough similar '{}' as a substitute.", req_fn, dta_fn.display());
                } else {
                    log::error!("No such file present as '{}'…", dta_fn.display());
                    misses += 1;
                    *req_fn = MISSING_FILE_MARKER.into();
                    continue;
                }
            }

            // convert DTA and store the result JSON…
            let dta = verify_and_categorize_dta(&dta_fn, read_lines(&dta_fn), args.verbose)
                .expect_left("Not an expected DTA file!");
            let gch_fn = format!("{}gch-{}.json",
                maybe_test_prefix(args.test),
                dta_fn.file_stem().unwrap().to_str().unwrap());
            fs::write(&gch_fn, serde_json::to_string_pretty(&dta).expect("FATAL: internal JSON debacle!"))
                .expect(&format!("Could not write '{}'!", gch_fn));
        }

        // Some logging based on misses and/or approximations or lack of both.
        match (misses, approx) {
            _ if misses > 0 && approx > 0 => log::warn!(
                "Genre '{}' {:.2}% complete (missing {misses} file{} out of {}); {approx} file{} approximated.",
                mf.name,
                to_percentage(misses as f64, mf.files.len() as f64),
                maybe_plural_s(misses), mf.files.len(),
                maybe_plural_s(approx)
            ),

            _ if misses > 0 => log::warn!(
                "Genre '{}' {:.2}% complete. Missing {misses} file{} out of {}.",
                mf.name,
                to_percentage(misses as f64, mf.files.len() as f64),
                maybe_plural_s(misses), mf.files.len()
            ),

            _ if approx > 0 => log::info!(
                "Genre '{}' 100% complete; {approx} file{} approximated.",
                mf.name, maybe_plural_s(approx)
            ),

            (_,_) => log::info!("Genre '{}' processed OK.", mf.name)
        }
    }

    // convert .dta entries to their .json variants
    for mf in mfpack.genres.iter_mut() {
        for dfn in mf.files.iter_mut() {
            if dfn == MISSING_FILE_MARKER {
                continue;
            }
            *dfn = format!("{}gch-{}", maybe_test_prefix(args.test), dfn.replace(".dta", ".json"))
        }
    }

    let gch_fname = mk_gch_manifest_filename(args.test);
    fs::write(&gch_fname, mfpack.to_string())
        .expect(&format!("FATAL: could not write '{gch_fname}'!"));
}

#[cfg(all(test, feature = "deep-tests"))]
mod main_tests {
    use gurpschgen_lib::dta::{locate_dta::locate_dta};

    use crate::{Cli, auto_generate_manifest_and_data};

    #[test]
    fn auto_test_works() {
        let args = Cli {
            path: None,// not used in this test
            test: true,
            auto: true,// not used in this test
            verbose: false,// not used in this test
            skip: vec![],// not used in this test
        };

        locate_dta(false);// autogen itself doesn't call locate_dta()
        auto_generate_manifest_and_data(&args);
    }
}
