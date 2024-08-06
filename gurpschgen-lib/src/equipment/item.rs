pub mod container;

use serde::{Deserialize, Serialize};

use crate::misc::{costly::Costly, mod_grouped::ModGrouped, named::Named, noted::Noted, skilled::Skilled, weighed::Weighed};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Item {
    pub name: String,
    pub notes: Option<String>,
    pub cost: Option<f64>,
    pub weight: Option<f64>,// most things have weight, but some have it so neglible that it's irrelevant.
    pub skill: Option<String>,// skill required/skill used with
    pub mod_groups: Vec<String>,
}

impl Costly for Item {
    fn cost(&self) -> f64 {
        match self.cost {
            Some(x) => x,
            _ => 0.0,
        }
    }
}

impl Named for Item {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Noted for Item {
    fn notes(&self) -> Option<&str> {
        self.notes.as_deref()
    }
}

impl Weighed for Item {
    fn weight(&self) -> Option<f64> {
        self.weight
    }
}

impl Skilled for Item {
    fn skill(&self) -> Option<&str> {
        self.skill.as_deref()
    }
}

impl ModGrouped for Item {
    fn mod_groups(&self) -> &Vec<String> {
        &self.mod_groups
    }
}
