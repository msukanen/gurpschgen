use regex::Regex;

use crate::misc::costly::Costly;

#[derive(Debug, Clone)]
pub struct Item {
    name: String,
    notes: Option<String>,
    cost: Option<f64>,
    weight: Option<f64>,// most things have weight, but some have it so neglible that it's irrelevant.
}

impl Costly for Item {
    fn cost(&self) -> f64 {
        match self.cost {
            Some(x) => x,
            _ => 0.0,
        }
    }
}

impl From<(&str, &str)> for Item {
    fn from(value: (&str, &str)) -> Self {
        let rx = Regex::new(r"^\s*(?<c1>[^;]*)?(;\s*((?<cost>\d+([.]?\d+)?)(\s*,\s*(?<wt>\d+([.]?\d+)?))?(;\s*((?<c3>[^;]*)?(;\s*((?<c4>[^;]*)?(;\s*(?<c5>[^;]*)?)?)?)?)?)?)?)?").unwrap();
        let mut notes = None;
        let mut cost = None;
        if let Some(caps) = rx.captures(value.1) {
            // notes
            if let Some(cap) = caps.name("c1") {
                let x = cap.as_str().trim();
                if !x.is_empty() {
                    notes = Some(x.to_string())
                }
            }
            
            // cost & weight
            if let Some(cap) = caps.name("cost") {
                let x = cap.as_str().trim();
                if !x.is_empty() {
                    cost = Some(x.parse::<f64>().unwrap())
                }
            }
        }

        Self {
            name: value.0.to_string(),
            notes,
            cost,
            weight: Some(0.0),
        }
    }
}

#[cfg(test)]
mod item_tests {
    use super::Item;

    #[test]
    fn full_item_works() {
        let raw = ("An Item", "notes;200.5");
        let item = Item::from(raw);
        assert_eq!("An Item", item.name);
        assert_eq!(200.5, item.cost.unwrap());
    }
}
