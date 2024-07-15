//!
//! Basic GURPS DTA/GEN file converter to convert those files into format `gurpschgen` understands.
//! 
//! Copyright © 2024 Markku Sukanen
//! 
//! DISCLAIMER: feel free to use the code as you see fit - be it for your own use,
//!             derivate work, commercial capacity or whatever else.
//! 
mod locate_dta;
mod parse_dta;
mod context;

use std::{fs::File, io::{BufRead, BufReader, Lines, Result}, path::{Path, PathBuf}};

use clap::Parser;
use locate_dta::locate_dta;
use parse_dta::parse_dta;

#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

fn main() {
    println!("GURPS .DTA/.GEN → JSON Converter");
    let args = Cli::parse();
    locate_dta();
    parse_dta(&args.path, read_lines(args.path.clone()));
}

/**
 Read lines from `filename`.

 **Returns** `Ok()` or `Err()`.
 */
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
