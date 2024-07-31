pub mod container;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::misc::{costly::Costly, mod_grouped::ModGrouped, named::Named, noted::Noted, skilled::Skilled, weighed::Weighed};

static RX_ITEM: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:^\s*(?<notes>[^;]*)?(?:;\s*(?:(?<cost>\d+([.]?\d+)?)(?:\s*,\s*(?<wt>\d+([.]?\d+)?))?(?:;\s*(?:(?<skill>[^;]*)?(?:;\s*((?:[^;]*)?(?:;\s*(?<modgr>[^;]*)?)?)?)?)?)?)?)?)").unwrap());
pub(crate) static RX_WT: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:(?<lbs>\d+)\s*lbs?[.]?)").unwrap());

#[derive(Debug, Clone)]
pub struct Item {
    name: String,
    notes: Option<String>,
    cost: Option<f64>,
    weight: Option<f64>,// most things have weight, but some have it so neglible that it's irrelevant.
    skill: Option<String>,// skill required/skill used with
    mod_groups: Vec<String>,
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

impl From<(&str, &str)> for Item {
    fn from(value: (&str, &str)) -> Self {
        let mut notes = None;
        let mut cost = None;
        let mut weight = None;
        let mut skill = None;
        let mut mod_groups = vec![];
        if let Some(caps) = RX_ITEM.captures(value.1) {
            // notes
            if let Some(cap) = caps.name("notes") {
                let x = cap.as_str().trim();
                if !x.is_empty() {
                    notes = Some(x.to_string())
                }
            }
            
            // cost
            if let Some(cap) = caps.name("cost") {
                let x = cap.as_str().trim();
                if !x.is_empty() {
                    cost = Some(x.parse::<f64>().unwrap())
                }
            }

            // wt.
            if let Some(cap) = caps.name("wt") {
                let x = cap.as_str().trim();
                if !x.is_empty() {
                    weight = Some(x.parse::<f64>().unwrap())
                }
            }

            // skill
            if let Some(cap) = caps.name("skill") {
                let x = cap.as_str().trim();
                if !x.is_empty() {
                    skill = Some(x.to_string())
                }
            }

            // modgr
            if let Some(cap) = caps.name("modgr") {
                for x in cap.as_str().split(",") {
                    let x = x.trim();
                    if !x.is_empty() {
                        mod_groups.push(x.to_string())
                    }
                }
            }
        };

        Self { name: value.0.to_string(), notes, cost, weight, skill, mod_groups, }
    }
}

#[cfg(test)]
mod item_tests {
    use super::Item;

    #[test]
    fn full_item_works() {
        let raw = ("An Item", "notes;200.5   , 66.6;  Bicycling ;     ; Item Mod 1, IT_x, Alpha Quality ; ; ; ");
        let item = Item::from(raw);
        assert_eq!("An Item", item.name);
        assert_eq!(200.5, item.cost.unwrap());
        assert_eq!(66.6, item.weight.unwrap());
        assert_eq!("Bicycling", item.skill.unwrap().as_str());
        assert_eq!(3, item.mod_groups.len());
    }
}
