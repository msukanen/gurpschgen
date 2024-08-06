use armor::Armor;
use item::Item;
use serde::{Deserialize, Serialize};
use weapon::Weapon;

use crate::misc::costly::Costly;

pub mod weapon;
pub mod armor;
pub mod item;

/**
 Various equipment types.
 */
#[derive(Debug, Clone, Deserialize, Serialize)]
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

impl Costly for Equipment {
    fn cost(&self) -> f64 {
        match self {
            Self::Armor(a) => a.cost(),
            Self::Item(a) => a.cost(),
            Self::Weapon(a) => a.cost(),
        }
    }
}
