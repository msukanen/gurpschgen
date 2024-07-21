use std::{collections::HashMap, fs::File, io::{BufReader, Lines, Result}, path::PathBuf};

use regex::Regex;

use crate::context::{CategoryPayload, Context};

const XCG_DATA_FORMAT: &'static str = "#XCG/DATA";
const STEVE_JACKSONS_FORMAT: &'static str = "GURPS data file (this MUST be the first line!)";

#[derive(Debug)]
pub(crate) struct Category {
    name: String,
    items: HashMap<String, CategoryPayload>,
}

impl Category {
    pub fn new(name: &str) -> Self {
        Category { name: name.to_string(), items: HashMap::new() }
    }
}

#[derive(Debug)]
pub(crate) struct Type {
    context: Context,
    items: HashMap<String, Category>,
}

impl Type {
    pub fn new(context: Context) -> Self {
        Type { context, items: HashMap::new() }
    }
}

/**
 Parse DTA lines.

 *dev NOTE:* As per "official" rules, if an [Item] is reintroduced, latest data overwrites the earlier item.

 **Params**
 * `filename` - presumed origin of the fed lines.
 * `lines` - DTA stuff, line per line.
 
 **Returns** items categorized; [Type] → [Category] → [Item] -tree.
 */
pub(crate) fn verify_and_categorize_dta(filename: &PathBuf, lines: Result<Lines<BufReader<File>>>, verbose: bool) -> HashMap<Context, Type> {
    if let Ok(lines) = lines {
        if verbose {println!("F: .dta/.gen {:?}", filename);}

        let mut curr_type: Option<Context> = None;
        let mut curr_category: String = String::from("");
        let mut unprocessed_items: HashMap<Context, Type> = HashMap::new();

        let rx_context_type = Regex::new(format!("^\\s*type\\s+({})\\s*$", [
            Context::Advantage.to_string(),
            Context::Bonus.to_string(),
            Context::Disadvantage.to_string(),
            Context::Equipment.to_string(),
            Context::Modifier.to_string(),
            Context::Package.to_string(),
            Context::Quirk.to_string(),
            Context::Skill.to_string(),
        ].join("|")).as_str()).unwrap();
        let rx_title = Regex::new(r"^(title|TITLE:)\s(.*)").unwrap();
        let rx_category = Regex::new(r"^\s*category\s(.*)").unwrap();
        let rx_item = Regex::new(r"^\s*([^;]+)([;]?(.*))\s*$").unwrap();
        let rx_whitespace = Regex::new(r"^\s*$").unwrap();

        for (file_line, line) in lines.flatten().enumerate() {
            let curr_line = file_line + 1;
            //
            // Detect file type. First line of file determines that.
            //
            if file_line == 0 {
                if line.eq(XCG_DATA_FORMAT) {
                    if verbose {println!(" → {} file format detected.", XCG_DATA_FORMAT)};
                } else if line.eq(STEVE_JACKSONS_FORMAT) {
                    if verbose {println!(" → GURPS MakeChar DTA file format detected.")};
                } else {
                    panic!("FATAL: unrecognized file format!")
                }
                continue;
            }
            //
            // Title?
            //
            if let Some(caps) = rx_title.captures(line.as_str()) {
                if verbose {println!("   \"{}\"", caps.get(2).unwrap().as_str());}
                continue;
            }

            // We skip all empty lines and lines which are considered comments.
            if line.starts_with("*")
            || line.is_empty()
            || rx_whitespace.is_match(line.as_str())
            {
                continue;
            }

            //
            // Context type change?
            //
            if let Some(caps) = rx_context_type.captures(line.as_str()) {
                curr_category.clear();// Clear current category upon type change.
                let typ = Context::from(caps.get(1).unwrap().as_str());
                if curr_type != Some(typ.clone()) {
                    curr_type = typ.clone().into();
                    if !unprocessed_items.contains_key(&typ) {
                        unprocessed_items.insert(typ.clone(), Type::new(typ));
                    }
                }
                
                if verbose {println!("T: {:?}", curr_type);}
                continue;
            }
            //
            // Category change?
            //
            if let Some(caps) = rx_category.captures(line.as_str()) {
                if curr_type.is_none() {
                    panic!("FATAL: \"category\" outside of a \"type\" on line {} in {}", curr_line, filename.display())
                }
                let cat_name = caps.get(1).unwrap().as_str();
                if !curr_category.eq(cat_name) {
                    curr_category = cat_name.to_string();
                    if let Some(typ) = unprocessed_items.get_mut(&curr_type.clone().unwrap()) {
                        if !typ.items.contains_key(cat_name) {
                            typ.items.insert(cat_name.to_string(), Category::new(cat_name));
                        }
                    }
                }

                if verbose {println!("C: {:?}", curr_category);}
                continue;
            }
            
            // Prevent orphaned non-type non-category entries.
            if curr_type.is_none() || curr_category.is_empty() {
                // note: "type bonus" associates all entries under one and the same [Category].
                if curr_type.eq(&Some(Context::Bonus)) {
                    curr_category = String::from("bonus");
                    if let Some(typ) = unprocessed_items.get_mut(&curr_type.clone().unwrap()) {
                        if !typ.items.contains_key("bonus") {
                            typ.items.insert(curr_category.clone(), Category::new(curr_category.as_str()));
                        }
                    }
                } else {
                    panic!("FATAL: entry outside of a \"type\" and/or \"category\" on line {} in {}", curr_line, filename.display())
                }
            }

            //
            // Other sort of a line...
            //
            if let Some(caps) = rx_item.captures(line.as_str()) {
                unprocessed_items.get_mut(&curr_type.clone().unwrap()).and_then(|typ|
                    typ.items.get_mut(curr_category.as_str()).and_then(|cat|{
                        let item_name = caps.get(1).unwrap().as_str().to_string();
                        if verbose {println!(" → {item_name}");}
                        cat.items.insert(item_name.clone(), CategoryPayload::from((&typ.context, item_name.as_str(), caps.get(2).unwrap().as_str())))
                    })
                );
            } else {
                panic!("No match?! {}", line.as_str())
            }
        }

        unprocessed_items
    } else {
        panic!("FATAL: we have a panic with '{}'. It could not be opened/found!", filename.display())
    }
}

#[cfg(test)]
mod parse_dta_tests {
    use std::path::PathBuf;

    use crate::dta::{locate_dta::locate_dta, read_lines::read_lines};

    use super::verify_and_categorize_dta;

    #[test]
    fn parse_starts_makechar_format() {
        locate_dta(true);
        let filename = PathBuf::from("test.dta");
        verify_and_categorize_dta(&filename, read_lines(&filename), true);
    }

    #[test]
    fn parse_starts_xcg_format() {
        locate_dta(true);
        let filename = PathBuf::from("test2.dta");
        verify_and_categorize_dta(&filename, read_lines(&filename), true);
    }

    #[test]
    #[should_panic]
    fn parse_panic_with_unrecognized_file() {
        locate_dta(true);
        let filename = PathBuf::from("test3.dta");
        verify_and_categorize_dta(&filename, read_lines(&filename), true);
    }

    #[test]
    fn parse_returned_hashmap_is_as_expected() {
        locate_dta(false);
        let filename = PathBuf::from("test.dta");
        let unprocessed = verify_and_categorize_dta(&filename, read_lines(&filename), false);
        for (k, v) in unprocessed {
            println!("k: {k}, v:{v:?}");
        }
    }
}
