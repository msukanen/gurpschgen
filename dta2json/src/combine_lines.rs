use std::io::{BufReader, Lines, Read, Result};

use regex::Regex;

/**
 Combine lines.
 */
pub fn combine_lines<R>(lines: Result<Lines<BufReader<R>>>) -> Vec<String>
where R: Sized + Read
{
    let rxline = Regex::new(r"^(?<line>.*)\\$").unwrap();
    if let Ok(lines) = lines {
        let mut combined_lines = vec![];
        let mut curr_line: String = "".into();
        for line in lines {
            let line = line.expect("FATAL: Something wrong in the neighborhood... or rather, a file error.");

            if let Some(x) = rxline.captures(line.as_str()) {
                curr_line.push_str(x.name("line").unwrap().as_str());
            } else {
                curr_line.push_str(&line);
                combined_lines.push(std::mem::take(&mut curr_line));
            }
        }
        combined_lines
    } else {vec![]}
}
