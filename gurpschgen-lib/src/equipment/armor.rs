use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{damage::{DamageResistance, PassiveDefense}, misc::{costly::HasCost, mod_grouped::HasModGroups, named::HasName, skilled::Skilled, weighed::Weighed}, attrib::AttributeType};

use super::item::container::Container;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Armor {
    pub name: String,
    pub dr: Option<DamageResistance>,
    pub pd: Option<PassiveDefense>,
    pub cover: HashSet<i32>,
    pub cost: Option<f64>,
    pub weight: Option<f64>,// most things have weight, but e.g. magic armor wt. might be neglible
    pub mod_groups: Vec<String>,
    pub skill: Option<String>,
    pub stats_affected: Vec<(AttributeType, i32)>,
    pub skills_affected: Vec<(String, i32)>,
    pub container: Option<Container>,
    pub _extra: Vec<String>,
}

impl HasCost for Armor {
    fn cost(&self) -> f64 {
        if let Some(x) = self.cost {x} else {0.0}
    }
}

impl Weighed for Armor {
    fn weight(&self) -> Option<f64> {
        self.weight
    }
}

impl HasName for Armor {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Skilled for Armor {
    fn skill(&self) -> Option<&str> {
        if let Some(x) = &self.skill {
            x.as_str().into()
        } else {
            None
        }
    }
}

impl Armor {
    /**
     Get the armor's DR ([damage resistance][DamageResistance]), if applicable.
     */
    pub fn dr(&self) -> &Option<DamageResistance> {
        &self.dr
    }

    /**
     Get the armor's PD ([passive defense][PassiveDefense]), if applicable.
     */
    pub fn pd(&self) -> &Option<PassiveDefense> {
        &self.pd
    }

    /**
     Get hit locations covered.

     **Returns** a (possibly empty) hash of covered hit locations.
     */
    pub fn cover(&self) -> &HashSet<i32> {
        &self.cover
    }
}

impl HasModGroups for Armor {
    fn mod_groups(&self) -> &Vec<String> {
        &self.mod_groups
    }
}
