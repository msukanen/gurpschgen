//!
//! Basic GURPS DTA/GEN file converter to convert those files into format `gurpschgen` understands.
//! 
//! Copyright © 2024 Markku Sukanen
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

use std::{collections::{HashMap, VecDeque}, path::PathBuf};

use clap::Parser;
use glob::{MatchOptions, glob_with};
use gurpschgen_lib::{context::{Context, ContextPayload}, dta::{filetype::LegacyFileExt, locate_dta::locate_dta, read_lines::read_lines}, misc::category::Category};
use once_cell::sync::Lazy;
use regex::Regex;
use verify_dta::verify_and_categorize_dta;

static RX_COST_WEIGHT: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:\s*(?<cost>\d+(?:[.]\d+)?)(?:\s*,\s*(?<wt>\d+(?:[.]\d+)?))?)").unwrap());

#[derive(Parser)]
struct Cli {
    path: Option<PathBuf>,
    #[arg(short,long)]
    verbose: bool,
    #[arg(short,long)]
    test: bool,
    #[arg(short,long)]
    batch: bool,
    #[arg(short,long)]
    skip: Option<String>,
}

fn main() {
    let args = Cli::parse();
    if args.verbose {println!("GURPS .DTA/.GEN → JSON Converter");}
    locate_dta(args.verbose);
    
    if !args.batch && args.path.is_none() {
        println!("You need to either specify name of a datafile to process, or use --batch flag if you want to crunch them all at once…");
        return;
    }
    
    let mut lib;
    if args.batch {
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

        let mut skip = VecDeque::new();
        // we'll intentionally skip:
        // - GENRE.DTA - we don't need to nor want to parse it.
        // - GRIMOIRE.DTA - it's of no interest for us, being just a swift lookup table with source book references, etc.
        skip.push_back("GENRE.DTA".into());
        skip.push_back("GRIMOIRE.DTA".into());
        if !args.test {
            // TEST3.DTA is intentionally malformed... Skip if not in --test mode.
            skip.push_back("TEST3.DTA".into());
        }
        
        if let Some(skips) = args.skip {
            let words: Vec<&str> = skips.split(",").collect();
            for word in words.iter() {
                skip.push_back(word.trim().to_uppercase());
            }
        }

        lib = HashMap::new();
        let mut glob_opt = MatchOptions::new();
        glob_opt.case_sensitive = false;
        for dtafname in glob_with(LegacyFileExt::AllDTA.as_str(), glob_opt).expect("Something wrong with glob?!") {
            if let Ok(path) = dtafname {
                let filename = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_uppercase();
                if skip.contains(&filename) {
                    continue;
                }

                if args.verbose || args.test {
                    println!("Processing {filename}");
                }
                merge_libs(&mut lib, verify_and_categorize_dta(&path, read_lines(path.clone()), args.verbose))
            }
        }
    } else {
        let path = args.path.unwrap();
        if args.verbose || args.test {
            println!("Processing {}", path.display());
        }
        lib = verify_and_categorize_dta(&path, read_lines(path.clone()), args.verbose);
    }

    if !args.test { println!("{}", serde_json::to_string(&lib).unwrap()); }
}

#[cfg(test)]
mod main_tests {
    use std::path::PathBuf;

    use gurpschgen_lib::dta::{locate_dta::locate_dta, read_lines::read_lines};

    use crate::verify_dta::verify_and_categorize_dta;

    #[test]
    fn x_dump_parsing_works() {
        let verbose = false;
        let path = PathBuf::from("_x.dump");
        locate_dta(verbose);
        let content = verify_and_categorize_dta(&path, read_lines(path.clone()), verbose);
        for x in content {
            println!("{}", x.0)
        }
    }
}
