use std::{fs::File, io::{BufReader, Lines, Result}, path::PathBuf};

pub(crate) fn parse_dta(filename: &PathBuf, lines: Result<Lines<BufReader<File>>>) {
    if let Ok(lines) = lines {
        println!("F: .dta/.gen {:?}", filename);
        for (curr_line, line) in lines.flatten().enumerate() {
            // we have two variants to (attempt to) detect file type:
            if curr_line == 0 {
                if line.eq("#XCG/DATA") {
                    println!("{}: XCG/DATA file format detected.", curr_line + 1);
                } else if line.eq("GURPS data file (this MUST be the first line!)") {
                    println!("{}: GURPS MakeChar DTA file format detected.", curr_line + 1);
                } else {
                    panic!("FATAL: unrecognized file format!")
                }
                continue;
            }
        }
    } else {
        panic!("FATAL: we have a panic with '{}'. It could not be opened/found!", filename.display())
    }
}

#[cfg(test)]
mod parse_dta_tests {
    use std::path::PathBuf;

    use crate::{locate_dta, read_lines};

    use super::parse_dta;

    #[test]
    fn parse_starts_makechar_format() {
        locate_dta();
        let filename = PathBuf::from("test.dta");
        parse_dta(&filename, read_lines(&filename));
    }

    #[test]
    fn parse_starts_xcg_format() {
        locate_dta();
        let filename = PathBuf::from("test2.dta");
        parse_dta(&filename, read_lines(&filename));
    }

    #[test]
    fn parse_panic_with_unrecognized_file() {
        locate_dta();
        let filename = PathBuf::from("test3.dta");
        parse_dta(&filename, read_lines(&filename));
    }
}
