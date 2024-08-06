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

use std::path::PathBuf;

use clap::Parser;
use gurpschgen_lib::dta::{locate_dta::locate_dta, read_lines::read_lines};
use once_cell::sync::Lazy;
use regex::Regex;
use verify_dta::verify_and_categorize_dta;

static RX_COST_WEIGHT: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:\s*(?<cost>\d+(?:[.]\d+)?)(?:\s*,\s*(?<wt>\d+(?:[.]\d+)?))?)").unwrap());

#[derive(Parser)]
struct Cli {
    path: PathBuf,
    verbose: Option<bool>,
}

fn main() {
    let args = Cli::parse();
    let verbose = if let Some(v) = args.verbose {v} else {false};
    if verbose {println!("GURPS .DTA/.GEN → JSON Converter");}
    locate_dta(verbose);
    let dump = verify_and_categorize_dta(&args.path, read_lines(args.path.clone()), verbose);
    println!("{}", serde_json::to_string(&dump).unwrap());
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
