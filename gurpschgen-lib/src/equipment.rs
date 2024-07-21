use armor::Armor;
use item::Item;
use regex::Regex;
use weapon::Weapon;

use crate::misc::costly::Costly;

pub mod weapon;
pub mod armor;
pub mod item;

/**
 Various equipment types.
 */
#[derive(Debug, Clone)]
pub enum Equipment {
    /**
     Armor goes here, with embedded data.
     */
    Armor(Armor),
    /**
     Generic items go here, with embedded data.
     */
    Item(Item),
    /**
     Weapons go here, with embedded data.
     */
    Weapon(Weapon),
}

impl From<(&str, &str)> for Equipment {
    fn from(value: (&str, &str)) -> Self {
        let rx_armor = Regex::new(r"(PD\s?\d|DR\s?\d)").unwrap();
        let rx_weapon = Regex::new(r"(SS\s?\d)").unwrap();
        
        // it's an armor?
        if let Some(_) = rx_armor.captures(value.1) {
            Self::Armor(Armor::from(value))
        }
        // it's a weapon?
        else if let Some(_) = rx_weapon.captures(value.1) {
            Self::Weapon(Weapon::from(value))
        }
        // nah... not armor or weapon, something else.
        else {
            Self::Item(Item::from(value))
        }
    }
}

impl Costly for Equipment {
    fn cost(&self) -> f64 {
        match self {
            Self::Armor(a) => a.cost(),
            Self::Item(a) => a.cost(),
            Self::Weapon(a) => a.cost(),
        }
    }
}
