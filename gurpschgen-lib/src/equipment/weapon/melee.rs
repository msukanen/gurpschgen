use serde::{Deserialize, Serialize};

use crate::{damage::{Damage, DamageDelivery}, misc::{costly::Costly, damaged::Damaged, noted::Noted, skilled::Skilled, st_req::STRequired, weighed::Weighed}};

/**
 Melee weapon data.
 */
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Melee {
    pub name: String,
    pub damage: Vec<Damage>,
    pub max_damage: Option<DamageDelivery>,
    pub cost: Option<f64>,
    pub weight: Option<f64>,
    pub skill: Option<String>,
    pub notes: Option<String>,
    pub mod_groups: Vec<String>,
    pub acc: Option<i32>,
    pub st_req: Option<i32>,
}

impl Costly for Melee {
    fn cost(&self) -> f64 {
        match self.cost {
            Some(x) => x,
            _ => 0.0
        }
    }
}

impl Noted for Melee {
    fn notes(&self) -> Option<&str> {
        if let Some(x) = &self.notes {
            x.as_str().into()
        } else {
            None
        }
    }
}

impl Weighed for Melee {
    fn weight(&self) -> Option<f64> {
        self.weight
    }
}

impl Skilled for Melee {
    fn skill(&self) -> Option<&str> {
        if let Some(x) = &self.skill {
            x.as_str().into()
        } else {
            None
        }
    }
}

impl STRequired for Melee {
    fn st_req(&self) -> &Option<i32> {
        &self.st_req
    }
}

impl Damaged for Melee {
    fn damage(&self) -> &Vec<Damage> {
        &self.damage
    }

    fn max_damage(&self) -> &Option<DamageDelivery> {
        &self.max_damage
    }
}
