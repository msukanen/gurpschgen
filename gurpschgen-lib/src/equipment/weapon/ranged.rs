use crate::{damage::Damage, misc::costly::Costly};

/**
 Ranged weapon data.
 */
#[derive(Debug, Clone)]
pub struct Ranged {
    name: String,
    damage: Vec<Damage>,
    cost: f64,
    weight: f64,
}

impl Costly for Ranged {
    fn cost(&self) -> f64 {
        self.cost
    }
}

impl From<(&str, &str)> for Ranged {
    fn from(value: (&str, &str)) -> Self {
        todo!("")
    }
}
