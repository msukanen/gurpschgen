//! "Genre" - game world/universe specific values.
//! 
//! # [GenreManifestPackage]
//! 
//! Contains all the (eligible) [genre manifests][GenreManifest].
//! 
//! # [GenreManifest]
//! 
//! Contains:
//! - name of genre
//! - description of genre
//! - default TL of genre (in average)
//! - data files used, in priority order from the lowest to the highest
//!   (later loaded data overrides earlier, when/if needed).
//! 
use std::{collections::HashMap, fmt::Display, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{context::{Context, ContextPayload}, misc::{category::Category, tl::TL}};

const fn default_max_attrskill() -> i32 {20}

/// Pre-vaulting runtime genre data goes here.
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

#[derive(Debug)]
pub enum GenreError {
    FileError(std::io::Error),
    JsonError(serde_json::Error),
    NoSuchGenre(String),
}

impl Display for GenreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileError(e) => write!(f, "{:?}", e),
            Self::JsonError(e) => write!(f, "{:?}", e),
            Self::NoSuchGenre(e) => write!(f, "No such genre found as '{e}'"),
        }
    }
}

impl From<std::io::Error> for GenreError {
    fn from(value: std::io::Error) -> Self {
        Self::FileError(value)
    }
}

impl From<serde_json::Error> for GenreError {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonError(value)
    }
}

impl TryFrom<&GenreManifest> for Genre {
    type Error = GenreError;

    fn try_from(value: &GenreManifest) -> Result<Self, Self::Error> {
        let items = try_load_and_merge_hashmaps(&value.files)?;

        Ok(Self {
            name: value.name.clone(),
            desc: value.desc.clone(),
            tl: TL::Exact(value.tl),
            max_attr_default: default_max_attrskill(),
            max_skill_default: default_max_attrskill(),
            files: value.files.clone(),
            items
        })
    }
}

fn try_load_and_merge_hashmaps(files: &Vec<String>) -> Result<HashMap<Context, ContextPayload>, GenreError> {
    let mut lib = HashMap::new();
    
    // Load each individual data file and combine their contents;
    // priority: newer overrides older, if/when necessary.
    for f in files {
        merge_genre_contents(&mut lib, serde_json::from_str(&fs::read_to_string(f)?)?);
    };

    Ok(lib)
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GenreManifest {
    pub name: String,
    pub desc: String,
    pub tl: u8,
    pub files: Vec<String>,
}

impl GenreManifest {
    /// Create a new [manifest][GenreManifest] based on **legacy** **MakeChar** **DTA** input.
    /// 
    /// # Args
    /// - `name_and_desc` oughta be some data that defines genre name and its description/title…
    /// 
    /// # Examples
    /// ```rust
    /// use gurpschgen_lib::dta::genre::GenreManifest;
    /// 
    /// fn some_func() {
    ///     let _ = GenreManifest::new_legacy("Some Genre : ...with descriptionless description!");
    /// }
    /// ```
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

impl TryFrom<&PathBuf> for GenreManifestPackage {
    type Error = GenreError;

    /// Load a [genre manifest package][GenreManifestPackage] file.
    /// 
    /// # Args
    /// - `manifest_fn` — the manifest's file name.
    /// 
    /// # Panic
    /// If the given `manifest_fn` is not found, panic ensues.
    fn try_from(manifest_fn: &PathBuf) -> Result<Self, GenreError> {
        Ok(serde_json::from_str::<Self>(&fs::read_to_string(&manifest_fn)?)?)
    }
}

impl GenreManifestPackage {
    pub fn find_genre(&self, name: &str) -> Result<Genre, GenreError> {
        self.genres.iter()
            .find(|m| m.name == name)
            .ok_or_else(|| GenreError::NoSuchGenre(name.into()))
            .and_then(|m| Genre::try_from(m).map_err(Into::into))
    }
}

/// Merge genre contents.
/// 
/// Entries within categories of `newer` replace those within `base`.
///
// We can't do this with simple '.extend()' call as we want to replace only
// the very deepest item(s) instead of whole branch(es).
pub fn merge_genre_contents(base: &mut HashMap<Context, ContextPayload>, newer: HashMap<Context, ContextPayload>) {
    for (context, newer_payload) in newer {
        let base_payload = base.entry(context.clone()).or_insert_with(|| ContextPayload { context, items: HashMap::new() });
        for (cat_name, newer_cat) in newer_payload.items {
            let base_cat = base_payload.items.entry(cat_name.clone())
                .or_insert_with(|| Category::new(&cat_name));
            base_cat.items.extend(newer_cat.items);
        }
    }
}

#[cfg(test)]
mod locate_dta_tests {
    use std::{collections::HashMap, str::FromStr};

    use crate::{dta::locate_dta::locate_dta, misc::tl::TL};

    use super::*;

    fn prepare() {
        let _ = env_logger::try_init();
        locate_dta(false);
    }

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

    /// Note that the tested values in this test rely on *unmodified* legacy `GENRE.DTA` contents!
    #[test]
    fn load_genre_works() {
        prepare();

        let genre_name = "Space";
        let package = GenreManifestPackage::try_from(&PathBuf::from_str("gch.manifest").unwrap()).unwrap_or_else(|e| panic!("{e:?}"));
        let manifest: &GenreManifest = package.genres.iter()
            .find(|mf| mf.name == genre_name)
            .expect(&format!("No manifest found for genre '{}'!", genre_name));
        let Ok(genre) = Genre::try_from(manifest) else {panic!("OMG!")};

        // these rely on facts present in *legacy* GENRE.DTA file…
        assert_eq!("Space", genre.name);
        assert_eq!("The Final Frontier (TL10)", genre.desc);
        assert_eq!(TL::Exact(10), genre.tl);
        assert_eq!(11, genre.files.len());
    }
}
