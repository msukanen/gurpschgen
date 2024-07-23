//!
//! Basic GURPS DTA/GEN file converter to convert those files into format `gurpschgen` understands.
//! 
//! Copyright © 2024 Markku Sukanen
//! 
//! DISCLAIMER: feel free to use the code as you see fit - be it for your own use,
//!             derivate work, commercial capacity or whatever else.
//! 
use std::path::PathBuf;

use clap::Parser;
use gurpschgen_lib::dta::{locate_dta::locate_dta, read_lines::read_lines, verify_dta::verify_and_categorize_dta};

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
    verify_and_categorize_dta(&args.path, read_lines(args.path.clone()), verbose);
}
