mod locate_dta;
mod parse_dta;

use std::{fs::File, io::{self, BufRead}, path::{Path, PathBuf}};

use clap::Parser;
use locate_dta::locate_dta;

#[derive(Parser)]
struct Cli {
    path: PathBuf,
}

fn main() {
    println!("GURPS .DTA/.GEN → JSON Converter");
    let args = Cli::parse();
    locate_dta();

    if let Ok(lines) = read_lines(args.path.clone()) {
        println!("F: .dta/.gen {:?}", args.path);
        for (curr_line, line) in lines.flatten().enumerate() {
            if curr_line == 0 && line.eq("#XCG/DATA") {
                println!("{}: {} is (probably) a valid XCG/DATA file.", curr_line + 1, args.path.display());
            }
        }
    } else {
        panic!("FATAL: we have a panic with '{}'. It could not be opened/found!", args.path.display())
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
