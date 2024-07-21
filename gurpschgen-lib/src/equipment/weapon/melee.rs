use crate::misc::costly::Costly;

use super::damage::Damage;

/**
 Melee weapon data.
 */
#[derive(Debug, Clone)]
pub struct Melee {
    name: String,
    damage: Vec<Damage>,
    cost: f64,
    weight: f64,
}

impl Costly for Melee {
    fn cost(&self) -> f64 {
        self.cost
    }
}

impl From<(&str, &str)> for Melee {
    fn from(value: (&str, &str)) -> Self {
        todo!("")
    }
}
