use once_cell::sync::Lazy;
use regex::Regex;

use armor::Armor;
use item::Item;
use weapon::Weapon;

use crate::misc::costly::Costly;
use armor::RX_IS_ARMOR;
use weapon::RX_SIMPLE_ANY_WPN;

pub(crate) static RX_TL: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:TL\s*(?<tl>\d+))").unwrap());
pub(crate) static RX_COUNTRY: Lazy<Regex> = Lazy::new(||Regex::new(r"US(SR)?|BE|GE|IT|IS|GR|UK|FI|SE|NO|INT").unwrap());
pub(crate) static RX_LEGALITY: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:LC\s*(?<lc>\d+))").unwrap());

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
        // it's an armor?
        if let Some(_) = RX_IS_ARMOR.captures(value.1) {
            Self::Armor(Armor::from(value))
        }
        // it's a weapon?
        else if let Some(_) = RX_SIMPLE_ANY_WPN.captures(value.1) {
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
