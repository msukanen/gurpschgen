use std::{collections::HashMap, path::PathBuf};

use glob::glob;
use serde::{Deserialize, Serialize};

use crate::{context::{Context, ContextPayload}, misc::tl::TL};

/**
 Genre data goes here.
 */
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Genre {
    pub name: String,
    pub title: String,
    pub tl: TL,
    pub max_attr_default: Option<i32>,
    pub max_skill_default: Option<i32>,
    pub files: Vec<String>,
    #[serde(skip)]
    pub items: HashMap<Context, ContextPayload>,
}

impl Genre {
    /**
     Generate a "template" genre with some "sensible" defaults.
     */
    pub fn new() -> Self {
        Self {
            name: String::from(""),
            title: String::from(""),
            tl: TL::Exact(3),
            max_attr_default: Some(20),
            max_skill_default: Some(20),
            files: vec![],
            items: HashMap::new(),
        }
    }

    /**
     Get maximum attribute default value.
     */
    pub fn max_attr_default(&self) -> i32 {
        match self.max_attr_default {
            None => 20,
            Some(x) => x
        }
    }

    /**
     Get maximum skill default value.
     */
    pub fn max_skill_default(&self) -> i32 {
        match self.max_skill_default {
            None => 20,
            Some(x) => x
        }
    }

    /**
     Load a genre from file.
     */
    pub fn load(filename: &PathBuf) -> Self {
        let mut genre: Genre = serde_json::from_str(
            &std::fs::read_to_string(filename).expect("Should have been able to read the file")
        ).expect("Error in JSON!");
        for f in &genre.files {
            let json = std::fs::read_to_string(f).expect(format!("Fail with {f}").as_str());
            let loaded_map: HashMap<Context, ContextPayload> = serde_json::from_str(&json).expect("Error in JSON!");
            // As simple .extend() doesn't suffice(?), we have to travel through the whole thing...
            for loaded_ct in loaded_map {
                if let Some(context_payload) = genre.items.get_mut(&loaded_ct.0) {
                    for loaded_ctg in loaded_ct.1.items {
                        if let Some(cat) = context_payload.items.get_mut(&loaded_ctg.0) {
                            cat.items.extend(loaded_ctg.1.items);
                        } else {
                            context_payload.items.insert(loaded_ctg.0.to_string(), loaded_ctg.1.clone());
                        }
                    }
                } else {
                    genre.items.insert(loaded_ct.0, loaded_ct.1);
                }
            }
        };
        genre
    }
}

/**
 Fetch a list of all ".genre" files.
 */
pub fn list_genre_files() -> Vec<PathBuf> {
    let mut gfs = vec![];
    for entry in glob("./*.genre").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => gfs.push(path),
            Err(e) => println!("{:?}", e)
        }
    }
    gfs
}

#[cfg(test)]
mod locate_dta_tests {
    use std::{collections::HashMap, env, path::PathBuf};

    use crate::{context::Context, dta::locate_dta::locate_dta, misc::{category::CategoryPayload, tl::TL}};

    use super::{list_genre_files, Genre};

    #[test]
    fn find_test_genres_works() {
        locate_dta(false);
        for g in list_genre_files() {
            println!("Found: {:?}", g.display())
        }
    }

    #[test]
    fn genre_to_json_works() {
        let g = Genre {
            name: "Basic Test".to_string(),
            title: "Basic test genre of genreness".to_string(),
            tl: TL::About { default: 3, min: 2, max: 4 },
            max_attr_default: Some(18),
            max_skill_default: None,
            files: vec![],
            items: HashMap::new(),
        };
        let json = serde_json::to_string(&g).unwrap();
        println!("{json}");
    }

    #[test]
    fn genre_from_json_works() {
        let json = r#"{
            "name": "Basic Test",
            "title": "Basically a basic test",
            "max_attr_default": 18,
            "max_skill_default": null,
            "tl": {"Exact": 3},
            "files": ["file.file", "file2.file"]
        }"#;
        let g: Genre = serde_json::from_str(json).unwrap();
        assert_eq!("Basic Test", g.name);
        assert_eq!("Basically a basic test", g.title);
        assert_eq!(TL::Exact(3), g.tl);
        assert_eq!(Some(18), g.max_attr_default);
        assert_eq!(None, g.max_skill_default);
    }

    #[test]
    fn load_genre_works() {
        let cwd = env::current_dir().unwrap();
        env::set_current_dir("../dta2json/datafiles").expect("?!");
        let f = PathBuf::from("test.genre");
        let g = Genre::load(&f);
        assert_eq!("Roleplaying in the world of The Final Frontier", g.title);
        env::set_current_dir(cwd).expect("!?");
        if let Some(a) = g.items.get(&Context::Advantage) {
            for x in &a.items {
                println!("{}", x.0)
            }
            if let Some(a) = a.items.get("Mental Advantages") {
                if let Some(a) = a.items.get("Empathy") {
                    match a {
                        CategoryPayload::Advantage(a) => println!("{} found!", a.name),
                        _ => panic!("WTF?")
                    }
                } else {
                    panic!("No E!")
                }
            } else {
                panic!("No MA!")
            }
        } else {
            panic!("No A!")
        }
    }
}
