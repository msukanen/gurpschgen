use std::{fs::File, io::{BufRead, BufReader, Lines, Result}, path::Path};
use regex::Regex;

/**
 Read lines from `filename`.

 **Returns** `Ok()` or `Err()`.
 */
pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

/**
 Combine lines.
 */
pub fn combine_lines(lines: Result<Lines<BufReader<File>>>) -> Vec<String> {
    let rxline = Regex::new(r"^(?<line>.*)\\$").unwrap();
    if let Ok(lines) = lines {
        let mut combined_lines = vec![];
        let mut curr_line = String::from("");
        for line in lines {
            if let Ok(line) = line {
                if let Some(x) = rxline.captures(line.as_str()) {
                    let l = x.name("line").unwrap().as_str();
                    if !curr_line.is_empty() {
                        curr_line += l
                    } else {
                        curr_line = l.to_string()
                    }
                } else {
                    if !curr_line.is_empty() {
                        curr_line += line.as_str();
                        combined_lines.push(curr_line);
                        curr_line = String::from("");
                    } else {
                        combined_lines.push(line)
                    }
                }
            } else {
                panic!("FATAL: Something wrong in the neighborhood... or rather, a file error.")
            }
        }
        combined_lines
    } else {vec![]}
}
