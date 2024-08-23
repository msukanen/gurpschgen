use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{adq::Adq, dta::genre::Genre, equipment::Equipment, skill::Skill};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Category {
    pub name: String,
    pub items: HashMap<String, CategoryPayload>,
}

impl Category {
    pub fn new(name: &str) -> Self {
        Category { name: name.to_string(), items: HashMap::new() }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CategoryPayload {
    Advantage(Adq),
    Bonus(String),
    Counter(String),
    Disadvantage(Adq),
    Equipment(Equipment),
    Genre(Genre),
    Modifier(String),
    Package(Adq),
    Quirk(String),
    Skill(Skill),
}
