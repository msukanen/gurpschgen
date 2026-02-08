use std::{collections::HashMap, io::{BufReader, Lines, Read, Result}, path::PathBuf};

use either::Either;
use gurpschgen_lib::{context::{Context, ContextPayload}, dta::{filetype::LegacyFileExt, genre::{Genre, GenreManifest, GenreManifestPackage}}, misc::{category::{Category, CategoryPayload}, tl::TL}};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{categorypayload::category_payload_from_triple, combine_lines::combine_lines, context::context_from_str};

const XCG_DATA_FORMAT: &'static str = "#XCG/DATA";
const STEVE_JACKSONS_FORMAT: &'static str = "GURPS data file (this MUST be the first line!)";
const GENRE_MANIFEST_FORMAT: &'static str = "1 The number before each list of files is the default TL of the genre";
//const STEVE_JACKSONS_GEN_FORMAT_RX: Lazy<Regex> = Lazy::new(||Regex::new(r"^(?:\s*\d\s+version\s+flag\s+(?<name>[^\n]+)\s+(?<title>[^\n]+)\s*(?:(?<default>\d+)\s+default\s*[tT][lL])?\s*(?:(?<min>\d+)\s+min\s+[tT][lL])?\s*(?:(?<max>\d+)\s+max\s+[tT][lL])?\s*(?:(?<attrmax>\d+)\s+[mM]ax(?:imum)?\s+attr[^\n]+)?\s*(?:(?<skillmax>\d+)\s+[mM]ax(?:imum)?\s+skill[^\n]+)?\s*(?<files>[\s\S]+)?)$").unwrap());

/// Parse DTA lines.
///
/// As per "official" rules, if an [Item] is reintroduced, latest data overwrites the earlier item.
/// 
/// # Args
/// - `filename`: presumed filename of the stuff's source… ;-)
/// - `lines`: DTA stuff, line-by-line.
///
/// # Returns
/// [Context]-indexed hashmap of [ContextPayload].
/// 
// This function is a monster, beware…!
pub fn verify_and_categorize_dta<R>(filename: &PathBuf, lines: Result<Lines<BufReader<R>>>, verbose: bool) -> Either<HashMap<Context, ContextPayload>, GenreManifestPackage>
where R: Sized + Read
{
    let lines = combine_lines(lines);
    if !lines.is_empty() {
        if verbose {
            println!("F: .{}/.{} {:?}",
                LegacyFileExt::DTA,
                LegacyFileExt::GEN,
                filename
            );
        }

        let mut curr_type: Option<Context> = None;
        let mut curr_category: String = "".into();
        let mut unprocessed_items: HashMap<Context, ContextPayload> = HashMap::new();
        let legacy_arab_dta = filename
            .file_stem().unwrap()
            .to_ascii_lowercase()
            .as_os_str() == "arab";

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
        let rx_gen_fmt = Regex::new(r"^(?:\s*\d+\s+version\s+flag)").unwrap();
        let rx_gen_tl = Regex::new(r"^(?:\s*(?<tl>\d+)\s+(?<mode>default|min|max)\s+[tT][lL])").unwrap();
        let rx_gen_attr = Regex::new(r"^(?:\s*(?<val>\d+)\s+[mM]ax(?:imum)\s+(?<mode>attr|skill))").unwrap();
        
        let mut genre: Lazy<Genre> = Lazy::new(Genre::default);
        
        let mut genre_manifest: Vec<GenreManifest> = vec![];
        let mut curr_manifest_genre: Option<GenreManifest> = None;

        let mut processing_gen_file = false;
        let mut processing_genre_manifest = false;

        for (linenr_0idx, data) in lines.iter().enumerate() {
            let curr_line = linenr_0idx + 1;
            //
            // Detect file type. First line of file determines that.
            //
            // However, if we're processing legacy ARAB.DTA, this doesn't apply
            // legacy ARAB.DTA doesn't begin with a proper file format specifier.
            //
            if linenr_0idx == 0 && legacy_arab_dta {
                if data.eq(STEVE_JACKSONS_FORMAT) {
                    // a fixed ARAB.DTA, who'd guessed that to happen?
                    if verbose {println!(" → GURPS MakeChar DTA file format detected.")};
                    continue;
                } else if data.eq("type bonus") {
                    // ye olde - pass forth.
                } else {
                    panic!("FATAL: ARAB.DTA, but not a recognized one…")
                }
            }
            else if linenr_0idx == 0 {
                if data.eq(XCG_DATA_FORMAT) {
                    if verbose {println!(" → {} file format detected.", XCG_DATA_FORMAT)};
                } else if data.eq(STEVE_JACKSONS_FORMAT) {
                    if verbose {println!(" → GURPS MakeChar DTA file format detected.")};
                } else if data.eq(GENRE_MANIFEST_FORMAT) {
                    if verbose {println!(" → GENRE manifest file detected.")};
                    processing_genre_manifest = true;
                } else if rx_gen_fmt.is_match(&data) {
                    if verbose {println!(" → GEN format genre file detected.")};
                    processing_gen_file = true;
                } else {
                    panic!("FATAL: unrecognized file format! {data}")
                }

                continue;

            } else if processing_gen_file {
                match linenr_0idx {
                    ..=1 => genre.name = data.to_string(),
                    2 => genre.desc = data.to_string(),
                    _ => if let Some(x) = rx_gen_tl.captures(&data) {
                        let (mut default, mut min, mut max) = match genre.tl {
                            TL::About { default, min, max } => (default, min, max),
                            TL::Exact(x) => (x,x,x)
                        };
                        let tl = x["tl"].parse::<i32>().unwrap();
                        match &x["mode"] {
                            "default" => default = tl as u8,
                            "min" => min = tl as u8,
                            "max" => max = tl as u8,
                            mode => unreachable!("Errorneous TL mode: \"{mode}\" on line {curr_line}?!")
                        }
                        genre.tl = TL::About { default, min, max }
                    } else if let Some(x) = rx_gen_attr.captures(&data) {
                        let val = x["val"].parse::<i32>().unwrap();
                        match &x["mode"] {
                            "attr" => genre.max_attr_default = val,
                            "skill" => genre.max_skill_default = val,
                            mode => unreachable!("Errorneous attr/skill mode: \"{mode}\" on line {curr_line}?!")
                        }
                    } else if !data.is_empty() && !rx_whitespace.is_match(data) {
                        // anything that didn't match a regex is a filename/list of filenames (8.3 letter MS-DOS format).
                        for fname in data.split(" ").into_iter() {
                            genre.files.push(fname.to_string())
                        }
                    }
                }

                // '*.genre' file has no other sorts of lines, ergo…
                continue;
            } else if processing_genre_manifest {
                match linenr_0idx % 2 {
                    // An odd linenum always lands on "title:desc" combination.
                    1 => {
                        // push earlier manifest into buffer, if present.
                        if let Some(old_mf) = curr_manifest_genre {
                            genre_manifest.push(old_mf);
                        }
                        if data.len() < 2 {
                            break;
                        }
                        curr_manifest_genre = Some(GenreManifest::new_legacy(data));
                    },
                    
                    _ => {
                        let parts = data.split_whitespace().collect::<Vec<&str>>();
                        if parts.len() < 2 {
                            panic!("Data \"{data}\" on line {curr_line} does not have whitespace separated TL that is followed by a filename list.");
                        }
                        let Some(m) = curr_manifest_genre.as_mut() else {panic!("We oughta had a manifest entry by now!")};
                        for (i,filename) in parts.iter().enumerate() {
                            match i {
                                0 => m.tl = filename.trim().parse::<u8>().expect(&format!("TL entry '{}' is not 0-255!", filename.trim())),
                                _ => m.files.push(filename.trim().into())
                            }
                        }
                    }
                }

                // genre manifest has no other sorts of lines, ergo…
                continue;
            }

            //
            // Title?
            //
            if let Some(caps) = rx_title.captures(data.as_str()) {
                if verbose {println!("   \"{}\"", caps.name("title").unwrap().as_str())}
                continue;
            }
            // Author?
            if let Some(caps) = rx_author.captures(data.as_str()) {
                if verbose {println!("    \"{}\"", caps.name("author").unwrap().as_str())}
                continue;
            }

            /*
             We skip all empty (or all-whitespace) lines and lines which are
             considered to be comments, e.g.:
               * a comment
               # another comment
            */
            if data.starts_with("*")
            || data.starts_with("#")
            || data.is_empty()
            || rx_whitespace.is_match(data.as_str())
            {
                continue;
            }

            //
            // Context type change?
            //
            if let Some(caps) = rx_context_type.captures(data.as_str()) {
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
            if let Some(caps) = rx_category.captures(&data) {
                if curr_type.is_none() {
                    panic!("FATAL: \"category\" outside of a \"type\" on line {} in {}", curr_line, filename.display())
                }
                
                let cat_name = &caps["cat"];
                if curr_category != cat_name {
                    curr_category = cat_name.to_string();
                    if let Some(typ) = unprocessed_items.get_mut(&curr_type.as_ref().unwrap()) {
                        typ.items
                            .entry(curr_category.clone())
                            .or_insert_with(|| Category::new(cat_name));
                    }
                }

                if verbose {println!("C: {:?}", curr_category);}
                continue;
            }
            
            // Prevent orphaned non-type non-category entries.
            if curr_type.is_none() || curr_category.is_empty() {
                match &curr_type {
                    Some(ct @ (Context::Bonus | Context::Counter)) => {
                        let cat_name = ct.to_string();
                        if let Some(typ) = unprocessed_items.get_mut(ct) {
                            typ.items.entry(cat_name.clone())
                                .or_insert_with(|| Category::new(&cat_name));
                        }

                        curr_category = cat_name
                    }

                    // truly orphaned data: no type nor category.
                    _ => {
                        eprintln!("--- Context: {curr_type:?}");
                        eprintln!("--- Data   : {data}");
                        panic!("FATAL: entry outside of any \"type\" and/or \"category\" on line {curr_line} in '{}'", filename.display());
                    }
                }
            }

            //
            // Other sort of a line...
            //
            if let Some(caps) = rx_item.captures(data.as_str()) {
                unprocessed_items.get_mut(&curr_type.clone().unwrap()).and_then(|typ|
                    typ.items.get_mut(curr_category.as_str()).and_then(|cat|{
                        let item_name = caps["name"].to_string();
                        if verbose {println!(" … {item_name} → {}", &caps["data"]);}
                        cat.items.insert(item_name.clone(), category_payload_from_triple((&typ.context, item_name.as_str(), &caps["data"])))
                    })
                );
            } else {
                panic!("No match?! {}", data.as_str())
            }
        }

        if processing_gen_file {
            unprocessed_items.insert(Context::Genre, ContextPayload {
                context: Context::Genre,
                items: {
                    let mut categorymap = HashMap::new();
                    let mut categorypayloadmap = HashMap::new();
                    categorypayloadmap.insert(Context::Genre.to_string(), CategoryPayload::Genre(genre.clone()));
                    categorymap.insert(Context::Genre.to_string(), Category { name: Context::Genre.to_string(), items: categorypayloadmap });
                    categorymap
                }
            });
        } else if processing_genre_manifest {
            return Either::Right(GenreManifestPackage { genres: genre_manifest })
        }
        
        Either::Left(unprocessed_items)
    } else {
        panic!("Something gone wrong with {:?}", filename.display())
    }
}

#[cfg(test)]
mod parse_dta_tests {
    use std::{collections::HashMap, io::{BufRead, BufReader, Cursor}, path::PathBuf};

    use either::Either;
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
        match dump {
            Either::Left(dump) => println!("{}", serde_json::to_string(&dump).unwrap()),
            _ => ()
        }
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
        if let Either::Left(gmap) = verify_and_categorize_dta(&filename, Ok(br), false) {
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
}
