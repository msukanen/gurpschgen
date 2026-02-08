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

use std::{collections::{HashMap, HashSet}, fs, path::PathBuf, str::FromStr};

use clap::Parser;
use either::Either;
use glob::{MatchOptions, glob_with};
use gurpschgen_lib::{context::{Context, ContextPayload}, dta::{filetype::LegacyFileExt, locate_dta::locate_dta, read_lines::read_lines}, misc::category::Category};
use once_cell::sync::Lazy;
use regex::Regex;
use verify_dta::verify_and_categorize_dta;

static RX_COST_WEIGHT: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:\s*(?<cost>\d+(?:[.]\d+)?)(?:\s*,\s*(?<wt>\d+(?:[.]\d+)?))?)").unwrap());
const MISSING_FILE_MARKER: &'static str = "<missing>";

pub(crate) const KNOWN_OFFENDER_FILES: [&'static str; 2] = [
    "GRIMOIRE.DTA",
    "TEST3.DTA",
];

#[derive(Parser)]
struct Cli {
    #[arg(conflicts_with = "test_dump")]
    path: Option<PathBuf>,
    
    #[arg(short,long)]
    verbose: bool,
    
    #[arg(long, conflicts_with = "auto")]
    test_dump: bool,
    
    #[arg(long, value_delimiter = ',')]
    skip: Vec<String>,
    
    #[arg(long, conflicts_with = "path")]
    auto: bool,
}

const fn to_percentage(x_of: f64, total: f64) -> f64 {
    ((total - x_of) / total) * 100.0
}

fn main() {
    let args = Cli::parse();
    let _ = env_logger::try_init();

    // pinpoint where DTAs live and move there
    locate_dta(args.verbose);
    
    if !args.auto && !args.test_dump && args.path.is_none() {
        println!("You need to either specify name of a datafile to process, or use --batch flag if you want to crunch them all at once…");
        return;
    }
    
    if args.test_dump {
        blob_all_dta(&args.skip, args.verbose);
    } else if args.auto {
        // first we deal with GENRE.DTA, if such is present, but if no such is found we bail out.
        let mut mfpack = verify_and_categorize_dta(&PathBuf::from_str("GENRE.DTA").unwrap(), read_lines("GENRE.DTA"), args.verbose)
            .expect_right("We didn't manage to deal with GENRE.DTA…");
        // let's deal with each individual DTA file mentioned…
        for mf in mfpack.genres.iter_mut() {
            let mut misses = 0;
            let mut approx = 0;
            for req_fn in mf.files.iter_mut() {
                let mut dta_fn = PathBuf::from_str(&req_fn).unwrap();
                if !dta_fn.exists() {
                    let stem = dta_fn.file_stem().unwrap().to_str().unwrap();
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
                        *req_fn = dta_fn.file_name().unwrap().to_str().unwrap().into();// crossing fingers here…
                        log::warn!("'{}' not found. Using enough similar '{}' as substitute.", req_fn, dta_fn.display());
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
                let gch_fn = format!("gch-{}.json", dta_fn.file_stem().unwrap().to_str().unwrap());
                fs::write(&gch_fn, serde_json::to_string_pretty(&dta).expect("FATAL: internal JSON debacle!"))
                    .expect(&format!("Could not write '{}'!", gch_fn));
            }
            
            match (misses, approx) {
                _ if misses > 0 && approx > 0 => log::warn!(
                    "Genre '{}' {:.2}% complete (missing {misses} file{} out of {}); {approx} file{} approximated.",
                    mf.name,
                    to_percentage(misses as f64, mf.files.len() as f64),
                    if misses == 1 {""} else {"s"}, mf.files.len(),
                    if approx == 1 {""} else {"s"}
                ),

                _ if misses > 0 => log::warn!(
                    "Genre '{}' {:.2}% complete. Missing {misses} file{} out of {}.",
                    mf.name,
                    to_percentage(misses as f64, mf.files.len() as f64),
                    if misses == 1 {""} else {"s"}, mf.files.len()
                ),

                _ if approx > 0 => log::info!(
                    "Genre '{}' 100% complete; {approx} file{} approximated.",
                    mf.name, if approx == 1 {""} else {"s"}
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
                *dfn = format!("gch-{}", dfn.replace(".dta", ".json"))
            }
        }

        fs::write("gch.manifest", mfpack.to_string())
            .expect("FATAL: could not write 'gch.manifest' file!");
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

/// Make a monolith dump of *all* (or nearly all) suitable DTA/GEN sources.
/// 
/// This will be printed into console and thus redirecting output manually is advised…
fn blob_all_dta(skip: &Vec<String>, verbose: bool) {
    fn merge_libs(base: &mut HashMap<Context, ContextPayload>, newer: HashMap<Context, ContextPayload>) {
        for (context, newer_payload) in newer {
            let base_payload = base.entry(context.clone()).or_insert_with(|| ContextPayload { context, items: HashMap::new() });
            for (cat_name, newer_cat) in newer_payload.items {
                let base_cat = base_payload.items.entry(cat_name.clone())
                    .or_insert_with(|| Category::new(&cat_name));
                base_cat.items.extend(newer_cat.items);
            }
        }
    }
    
    // uppercase all --skip defined file names.
    let skip_list: HashSet<String> = skip.iter()
        .map(|s| s.trim().to_uppercase())
        .chain(KNOWN_OFFENDER_FILES.iter().map(|s| s.to_string()))
        .collect();
    
    let mut lib = HashMap::new();
    let mut glob_opt = MatchOptions::new();
    glob_opt.case_sensitive = false;
    for dtafname in glob_with(LegacyFileExt::AllDTA.as_str(), glob_opt).expect("Something wrong with glob?!") {
        if let Ok(path) = dtafname {
            let filename = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_uppercase();
            if skip_list.contains(&filename) {
                continue;
            }

            log::info!("Processing {filename}");

            match verify_and_categorize_dta(&path, read_lines(path.clone()), verbose) {
                Either::Left(dta) => merge_libs(&mut lib, dta),
                // note that we do nothing with a manifest file at this point.
                _ => ()
            }
        }
    }

    // final pretty print as JSON
    println!("{}", serde_json::to_string_pretty(&lib).unwrap())
}

#[cfg(test)]
mod main_tests {
    use std::{collections::HashMap, fs, path::PathBuf};

    use gurpschgen_lib::{context::{Context, ContextPayload}, dta::{locate_dta::locate_dta}};

    #[test]
    fn x_dump_parsing_works() {
        let verbose = false;
        let path = PathBuf::from("_x.dump");
        locate_dta(verbose);
        let lib: HashMap<Context, ContextPayload> = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
        println!("{}", serde_json::to_string_pretty(&lib).unwrap());
    }
}
