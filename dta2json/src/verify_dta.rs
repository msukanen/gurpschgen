use std::{collections::HashMap, fs::File, io::{BufReader, Lines, Result}, path::PathBuf};

use gurpschgen_lib::{context::Context, misc::{category::Category, typing::Type}};
use regex::Regex;

use crate::{categorypayload::category_payload_from_triple, combine_lines::combine_lines, context::context_from_str};

const XCG_DATA_FORMAT: &'static str = "#XCG/DATA";
const STEVE_JACKSONS_FORMAT: &'static str = "GURPS data file (this MUST be the first line!)";

/**
 Parse DTA lines.

 *dev NOTE:* As per "official" rules, if an [Item] is reintroduced, latest data overwrites the earlier item.

 **Params**
 * `filename` - presumed origin of the fed lines.
 * `lines` - DTA stuff, line per line.
 
 **Returns** items categorized; [Type] → [Category] → [Item] -tree.
 */
pub fn verify_and_categorize_dta(filename: &PathBuf, lines: Result<Lines<BufReader<File>>>, verbose: bool) -> HashMap<Context, Type> {
    let lines = combine_lines(lines);
    if !lines.is_empty() {
        if verbose {println!("F: .dta/.gen {:?}", filename);}

        let mut curr_type: Option<Context> = None;
        let mut curr_category: String = String::from("");
        let mut unprocessed_items: HashMap<Context, Type> = HashMap::new();

        let rx_context_type = Regex::new(format!(r"^\s*type\s+({})\s*$", [
            Context::Advantage.to_string(),
            Context::Bonus.to_string(),
            Context::Counter.to_string(),
            Context::Disadvantage.to_string(),
            Context::Equipment.to_string(),
            Context::Modifier.to_string(),
            Context::Package.to_string(),
            Context::Quirk.to_string(),
            Context::Skill.to_string(),
            Context::Spell.to_string(),
        ].join("|")).as_str()).unwrap();
        let rx_title = Regex::new(r"^(?:\s*(?:title|TITLE:)\s+(?<title>.*))").unwrap();
        let rx_author = Regex::new(r"^(?:\s*(?:author|Author|AUTHOR):?\s*(?<author>.*))").unwrap();
        let rx_category = Regex::new(r"^(?:\s*category\s(?<cat>.*))").unwrap();
        let rx_item = Regex::new(r"^(?:\s*(?<name>[^;]+)(?:;?\s*(?<data>.*)?)?)").unwrap();
        let rx_whitespace = Regex::new(r"^(\s|)*$").unwrap();

        for (file_line, line) in lines.iter().enumerate() {
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
                    panic!("FATAL: unrecognized file format! {line}")
                }
                continue;
            }
            //
            // Title?
            //
            if let Some(caps) = rx_title.captures(line.as_str()) {
                if verbose {println!("   \"{}\"", caps.name("title").unwrap().as_str())}
                continue;
            }
            // Author?
            if let Some(caps) = rx_author.captures(line.as_str()) {
                if verbose {println!("    \"{}\"", caps.name("author").unwrap().as_str())}
                continue;
            }

            /*
             We skip all empty (or all-whitespace) lines and lines which are
             considered to be comments, e.g.:
               * a comment
               # another comment
            */
            if line.starts_with("*")
            || line.starts_with("#")
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
                let typ = context_from_str(caps.get(1).unwrap().as_str());
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
                if curr_type.eq(&Some(Context::Bonus))
                || curr_type.eq(&Some(Context::Counter))
                {
                    let ct = curr_type.clone().unwrap().to_string();
                    curr_category = String::from(&ct);
                    if let Some(typ) = unprocessed_items.get_mut(&curr_type.clone().unwrap()) {
                        if !typ.items.contains_key(&ct) {
                            typ.items.insert(curr_category.clone(), Category::new(curr_category.as_str()));
                        }
                    }
                } else {
                    println!("--- {}", line.as_str());
                    panic!("FATAL: entry outside of a \"type\" and/or \"category\" on line {} in {}", curr_line, filename.display());
                }
            }

            //
            // Other sort of a line...
            //
            if let Some(caps) = rx_item.captures(line.as_str()) {
                unprocessed_items.get_mut(&curr_type.clone().unwrap()).and_then(|typ|
                    typ.items.get_mut(curr_category.as_str()).and_then(|cat|{
                        let item_name = caps.name("name").unwrap().as_str().to_string();
                        if verbose {println!("› {item_name} → {}", caps.name("data").unwrap().as_str());}
                        cat.items.insert(item_name.clone(), category_payload_from_triple((&typ.context, item_name.as_str(), caps.name("data").unwrap().as_str())))
                    })
                );
            } else {
                panic!("No match?! {}", line.as_str())
            }
        }

        unprocessed_items
    } else {
        panic!("Something gone wrong with {:?}", filename.display())
    }
}

#[cfg(test)]
mod parse_dta_tests {
    use std::{collections::HashMap, path::PathBuf};

    use gurpschgen_lib::{context::{CategoryPayload, Context}, damage::{Damage, DamageDelivery}, dta::{locate_dta::locate_dta, read_lines::read_lines}, equipment::{weapon::{ranged::{rof::RoF, shots::{Battery, Shots}, Ranged}, Weapon}, Equipment}, misc::{category::Category, typing::Type}};

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
        locate_dta(true);
        let filename = PathBuf::from("_x.dump");
        let dump = verify_and_categorize_dta(&filename, read_lines(&filename), true);
        println!("{}", serde_json::to_string(&dump).unwrap());
    }

    #[test]
    fn serde_type_works() {
        let mut genre = HashMap::new();
        let mut items = HashMap::new();
        let mut cat_items = HashMap::new();
        cat_items.insert("A thing".to_string(), CategoryPayload::Equipment(Equipment::Weapon(Weapon::Ranged(Ranged {
            name: "A thing".to_string(),
            damage: vec![Damage::Var(DamageDelivery::DiceMul(3, 2, 1.5))],
            max_damage: None,
            acc: 5, ss: Some(12), rof: RoF::SemiAuto(3).into(), rcl: None,
            min_range: None, half_dmg_range: Some(50), max_range: Some(150),
            st_req: None, tripod: false, cost: Some(125.75), weight: Some(2.25),
            skill: "Thing Weapon".to_string().into(), notes: Some("This is a note".to_string()),
            shots: Some(Shots::Battery(50, Battery::C)), mod_groups: vec!["Lazoring".to_string()],
            rl_year: None, rl_country: None, tl: Some(8), lc: Some(0)
        }))));
        let cat = Category {
            name: "Things".to_string(),
            items: cat_items,
        };
        items.insert("Things".to_string(), cat);
        let t = Type {
            context: Context::Equipment,
            items,
        };
        genre.insert(Context::Equipment, t.clone());
        genre.insert(Context::Bonus, t);
        let json = serde_json::to_string(&genre).unwrap();
        println!("JSON:\n{json}\n");
        let g: HashMap<Context, Type> = serde_json::from_str(&json).unwrap();
        println!("UnJSON:\n{:?}", g);
    }
}
