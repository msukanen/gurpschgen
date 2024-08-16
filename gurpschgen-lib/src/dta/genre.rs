use std::path::PathBuf;

use glob::glob;
use serde::{Deserialize, Serialize};

use crate::misc::tl::TL;

/**
 Genre data goes here.
 */
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Genre {
    pub name: String,
    pub title: String,
    pub tl: TL,
    pub max_attr_default: Option<i32>,
    pub max_skill_default: Option<i32>,
    pub files: Vec<String>,
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
            files: vec![]
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
    use crate::{dta::locate_dta::locate_dta, misc::tl::TL};

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
            files: vec![]
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
        // TODO: using hardcoded path because for some reason locate_dta() misfires with 'cargo test'.
        if let Ok(json) = std::fs::read_to_string("../../dta2json/datafiles/test.genre") {
            let g: Genre = serde_json::from_str(&json).unwrap();
            assert_eq!("Space", g.name);
            assert_eq!("Roleplaying in the world of The Final Frontier", g.title);
            assert_eq!(TL::About { default: 10, min: 7, max: 10 }, g.tl);
            assert_eq!(Some(20), g.max_attr_default);
            assert_eq!(Some(40), g.max_skill_default);
            assert!(g.files.contains(&("basic.dta".to_string())));
            assert!(g.files.contains(&("spacenav.dta".to_string())));
        }
    }
}
