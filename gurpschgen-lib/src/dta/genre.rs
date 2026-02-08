//! "Genre" - game world/universe specific values.
use std::{collections::HashMap, fmt::Display, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{context::{Context, ContextPayload}, misc::tl::TL};

const fn default_max_attrskill() -> i32 {20}

/// Genre data goes here.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Genre {
    pub name: String,
    pub desc: String,
    pub tl: TL,
    #[serde(default = "default_max_attrskill")]
    pub max_attr_default: i32,
    #[serde(default = "default_max_attrskill")]
    pub max_skill_default: i32,
    
    pub files: Vec<String>,
    #[serde(skip)]
    pub items: HashMap<Context, ContextPayload>,
}

impl Default for Genre {
    fn default() -> Self {
        Self {
            name: String::from(""),
            desc: String::from(""),
            tl: TL::Exact(3),
            max_attr_default: default_max_attrskill(),
            max_skill_default: default_max_attrskill(),
            files: vec![],
            items: HashMap::new(),
        }
    }
}

impl From<&PathBuf> for Genre {
    fn from(filename: &PathBuf) -> Self {
        Genre::load(filename)
    }
}

impl Genre {
    /// Load a genre from `filename`.
    pub fn load(filename: &PathBuf) -> Self {
        let mut genre: Genre = serde_json::from_str(
            &std::fs::read_to_string(filename).expect("Should have been able to read the file")
        ).expect("Error in JSON!");
        for f in &genre.files {
            let json = std::fs::read_to_string(f).expect(format!("Fail with {f}").as_str());
            let loaded_map: HashMap<Context, ContextPayload> = serde_json::from_str(&json).expect("Error in JSON!");
            // As simple `.extend()` call doesn't work here, we have to traverse manually…
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GenreManifest {
    pub name: String,
    pub desc: String,
    pub tl: u8,
    pub files: Vec<String>,
}

impl GenreManifest {
    /// Create new manifest based on **legacy** **MakeChar** **DTA** input.
    /// 
    /// # Args
    /// - `name_and_desc` oughta be some data that defines genre name and its description/title…
    pub fn new_legacy(name_and_desc: &str) -> Self {
        let (name, desc) = name_and_desc.split_once(':')
            .expect(format!("FATAL: Legacy Genre format requires genre title to be separated from genre description with ':'.\nNo such present in: \"{name_and_desc}\"").as_str());

        Self {
          name: name.trim().into(),
          desc: desc.trim().into(),
          tl: 0,
          files: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GenreManifestPackage {
    pub genres: Vec<GenreManifest>
}

impl Display for GenreManifestPackage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

impl GenreManifestPackage {
    /// Load 'genre manifest'.
    pub fn load(manifest_fn: &str) -> Self {
        let path = PathBuf::from(manifest_fn);
        serde_json::from_str(
            &fs::read_to_string(&path)
                .expect(&format!("Could not open '{}'", path.display()).as_str())
        ).expect("JSON borked!")
    }
}

#[cfg(test)]
mod locate_dta_tests {
    use std::{collections::HashMap, env, fs};

    use crate::{context::{Context, ContextPayload}, misc::tl::TL};

    use super::*;

    #[test]
    fn genre_to_json_works() {
        let g = Genre {
            name: "Basic Test".to_string(),
            desc: "Basic test genre of genreness".to_string(),
            tl: TL::About { default: 3, min: 2, max: 4 },
            max_attr_default: 18,
            max_skill_default: default_max_attrskill(),
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
            "desc": "Basically a basic test",
            "max_attr_default": 18,
            "tl": {"Exact": 3},
            "files": ["file.file", "file2.file"]
        }"#;
        let g: Genre = serde_json::from_str(json).unwrap();
        assert_eq!("Basic Test", g.name);
        assert_eq!("Basically a basic test", g.desc);
        assert_eq!(TL::Exact(3), g.tl);
        assert_eq!(18, g.max_attr_default);
        assert_eq!(20, g.max_skill_default);
    }

    #[test]
    fn load_genre_works() {
        let _ = env_logger::try_init();
        //let cwd = env::current_dir().unwrap();
        env::set_current_dir("../dta2json/datafiles").expect("?!");
        let genre_name = "Space";
        let package = GenreManifestPackage::load("gch.manifest");
        let manifest: &GenreManifest = package.genres.iter().find(|mf| mf.name == genre_name)
            .expect(&format!("No manifest found for genre '{}'!", genre_name));
        for f in manifest.files.iter() {
            log::info!("F: {f}");
            let lib: HashMap<Context, ContextPayload> = serde_json::from_str(
                &fs::read_to_string(f).expect("Uh oh...")
            ).expect("JSON in fire!");
            log::debug!("JSON…\n{}", serde_json::to_string_pretty(&lib).expect("JSON melted the CPU?"));
        }
    }
}
