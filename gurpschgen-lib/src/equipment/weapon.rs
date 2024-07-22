use melee::Melee;
use ranged::Ranged;
use regex::Regex;

use crate::misc::costly::Costly;

pub mod melee;
pub mod ranged;

#[derive(Debug, Clone)]
pub enum Weapon {
    Melee(Melee),
    Ranged(Ranged),
}

impl Costly for Weapon {
    fn cost(&self) -> f64 {
        match self {
            Self::Melee(a) => a.cost(),
            Self::Ranged(a) => a.cost(),
        }
    }
}

impl From<(&str, &str)> for Weapon {
    fn from(value: (&str, &str)) -> Self {
        let rx_ranged = Regex::new(r"SS\s*\d").unwrap();
        if let Some(_) = rx_ranged.captures(value.1) {
            Self::Ranged(Ranged::from(value))
        } else {
            Self::Melee(Melee::from(value))
        }
    }
}
