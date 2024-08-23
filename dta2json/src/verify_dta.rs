use std::{collections::HashMap, io::{BufReader, Lines, Read, Result}, path::PathBuf};

use gurpschgen_lib::{context::{Context, ContextPayload}, dta::genre::Genre, misc::{category::{Category, CategoryPayload}, tl::TL}};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{categorypayload::category_payload_from_triple, combine_lines::combine_lines, context::context_from_str};

const XCG_DATA_FORMAT: &'static str = "#XCG/DATA";
const STEVE_JACKSONS_FORMAT: &'static str = "GURPS data file (this MUST be the first line!)";
//const STEVE_JACKSONS_GEN_FORMAT_RX: Lazy<Regex> = Lazy::new(||Regex::new(r"^(?:\s*\d\s+version\s+flag\s+(?<name>[^\n]+)\s+(?<title>[^\n]+)\s*(?:(?<default>\d+)\s+default\s*[tT][lL])?\s*(?:(?<min>\d+)\s+min\s+[tT][lL])?\s*(?:(?<max>\d+)\s+max\s+[tT][lL])?\s*(?:(?<attrmax>\d+)\s+[mM]ax(?:imum)?\s+attr[^\n]+)?\s*(?:(?<skillmax>\d+)\s+[mM]ax(?:imum)?\s+skill[^\n]+)?\s*(?<files>[\s\S]+)?)$").unwrap());

/**
 Parse DTA lines.

 *dev NOTE:* As per "official" rules, if an [Item] is reintroduced, latest data overwrites the earlier item.

 **Params**
 * `filename` - presumed origin of the fed lines.
 * `lines` - DTA stuff, line per line.
 
 **Returns** items categorized; [Type] → [Category] → [Item] -tree.
 */
pub fn verify_and_categorize_dta<R>(filename: &PathBuf, lines: Result<Lines<BufReader<R>>>, verbose: bool) -> HashMap<Context, ContextPayload>
where R: Sized + Read
{
    let lines = combine_lines(lines);
    if !lines.is_empty() {
        if verbose {println!("F: .dta/.gen {:?}", filename);}

        let mut curr_type: Option<Context> = None;
        let mut curr_category: String = String::from("");
        let mut unprocessed_items: HashMap<Context, ContextPayload> = HashMap::new();

        let rx_whitespace = Regex::new(r"^(\s|)*$").unwrap();
        // DTA regexes
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
        // GEN regexes
        let rx_genre_fmt = Regex::new(r"^(?:\s*\d+\s+version\s+flag)").unwrap();
        let rx_genre_tl = Regex::new(r"^(?:\s*(?<tl>\d+)\s+(?<mode>default|min|max)\s+[tT][lL])").unwrap();
        let rx_genre_attr = Regex::new(r"^(?:\s*(?<val>\d+)\s+[mM]ax(?:imum)\s+(?<mode>attr|skill))").unwrap();
        
        let mut genre: Lazy<Genre> = Lazy::new(Genre::new);
        let mut processing_genre = false;

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
                } else if rx_genre_fmt.is_match(&line) {
                    //curr_type = Context::Genre.into();
                    //curr_category = Context::Genre.to_string();
                    processing_genre = true;
                } else {
                    panic!("FATAL: unrecognized file format! {line}")
                }
                continue;
            } else if processing_genre {
                match file_line {
                    ..=1 => genre.name = line.to_string(),
                    2 => genre.title = line.to_string(),
                    n => if let Some(x) = rx_genre_tl.captures(&line) {
                        let (mut default, mut min, mut max) = match genre.tl {
                            TL::About { default, min, max } => (default, min, max),
                            TL::Exact(x) => (x,x,x)
                        };
                        let tl = x.name("tl").unwrap().as_str().parse::<i32>().unwrap();
                        match x.name("mode").unwrap().as_str() {
                            "default" => default = tl,
                            "min" => min = tl,
                            "max" => max = tl,
                            m => unreachable!("Errorneous TL mode: \"{m}\" on line {n}?!")
                        }
                        genre.tl = TL::About { default, min, max }
                    } else if let Some(x) = rx_genre_attr.captures(&line) {
                        let val = x.name("val").unwrap().as_str().parse::<i32>().unwrap();
                        match x.name("mode").unwrap().as_str() {
                            "attr" => genre.max_attr_default = Some(val),
                            "skill" => genre.max_skill_default = Some(val),
                            m => unreachable!("Errorneous attr/skill mode: \"{m}\" on line {n}?!")
                        }
                    } else if !line.is_empty() && !rx_whitespace.is_match(line) {
                        // anything that didn't match a regex is a filename/list of filenames (8.3 letter MS-DOS format).
                        for fname in line.split(" ").into_iter() {
                            genre.files.push(fname.to_string())
                        }
                    }
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
                        unprocessed_items.insert(typ.clone(), ContextPayload::new(typ));
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

        if processing_genre {
            unprocessed_items.insert(Context::Genre, ContextPayload { context: Context::Genre, items: {
                let mut categorymap = HashMap::new();
                let mut categorypayloadmap = HashMap::new();
                categorypayloadmap.insert(Context::Genre.to_string(), CategoryPayload::Genre(genre.clone()));
                categorymap.insert(Context::Genre.to_string(), Category { name: Context::Genre.to_string(), items: categorypayloadmap });
                categorymap
            } });
        }
        
        unprocessed_items
    } else {
        panic!("Something gone wrong with {:?}", filename.display())
    }
}

#[cfg(test)]
mod parse_dta_tests {
    use std::{collections::HashMap, io::{BufRead, BufReader, Cursor}, path::PathBuf};

    use gurpschgen_lib::{context::{Context, ContextPayload}, damage::{Damage, DamageDelivery}, dta::{locate_dta::locate_dta, read_lines::read_lines}, equipment::{weapon::{ranged::{rof::RoF, shots::{Battery, Shots}, Ranged}, Weapon}, Equipment}, misc::{category::{Category, CategoryPayload}, tl::TL}};

    use super::verify_and_categorize_dta;
    //use super::STEVE_JACKSONS_GEN_FORMAT_RX;

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
        let t = ContextPayload {
            context: Context::Equipment,
            items,
        };
        genre.insert(Context::Equipment, t.clone());
        genre.insert(Context::Bonus, t);
        let json = serde_json::to_string(&genre).unwrap();
        println!("JSON:\n{json}\n");
        let g: HashMap<Context, ContextPayload> = serde_json::from_str(&json).unwrap();
        println!("UnJSON:\n{:?}", g);
    }

    #[test]
    fn parse_gen_works() {
        let raw = r"2    version flag
            SPACE
            Roleplaying in the world of The Final Frontier

            10    default TL
            7    min TL
            10   max TL
            20   Maximum attribute value from which a skill can default
            40   Maximum skill value from which a skill can default
            basic.dta tl10basi.dta optbasic.dta humannat.dta psionics.dta martial.dta spacenav.dta tl10equi.dta tl9equip.dta tl8equip.dta tl7equip.dta aliens.dta 
        ";
        let cursor = Cursor::new(raw);
        let br = BufReader::new(cursor).lines();
        let mut filename = PathBuf::new();
        filename.set_file_name("parse_gen_works");
        let gmap = verify_and_categorize_dta(&filename, Ok(br), false);
        if let Some(g) = gmap.get(&Context::Genre) {
            if let Some(i) = g.items.get("genre") {
                if let Some(p) = i.items.get("genre") {
                    match p {
                        CategoryPayload::Genre(g) => {
                            match g.tl {
                                TL::About { default, min, max } => {
                                    assert_eq!(10, default);
                                    assert_eq!(7, min);
                                    assert_eq!(10, max);
                                },
                                _ => panic!("{:?} should've been TL::About{{}}", g.tl)
                            }
                        },
                        _ => panic!("Not a genre?!")
                    }
                }
            }
        }
    }
}
