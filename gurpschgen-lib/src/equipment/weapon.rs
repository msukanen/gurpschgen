use melee::Melee;
use ranged::Ranged;
use serde::{Deserialize, Serialize};

use crate::{damage::{Damage, DamageDelivery}, misc::{costly::Costly, damaged::Damaged, st_req::STRequired}};

pub mod melee;
pub mod ranged;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Weapon {
    Melee(Melee),
    Ranged(Ranged),
}

impl STRequired for Weapon {
    fn st_req(&self) -> &Option<i32> {
        match self {
            Self::Melee(x) => x.st_req(),
            Self::Ranged(x) => x.st_req(),
        }
    }
}

impl Costly for Weapon {
    fn cost(&self) -> f64 {
        match self {
            Self::Melee(a) => a.cost(),
            Self::Ranged(a) => a.cost(),
        }
    }
}

impl Damaged for Weapon {
    fn damage(&self) -> &Vec<Damage> {
        match self {
            Self::Melee(x) => x.damage(),
            Self::Ranged(x) => x.damage()
        }
    }

    fn max_damage(&self) -> &Option<DamageDelivery> {
        match self {
            Self::Melee(x) => x.max_damage(),
            Self::Ranged(x) => x.max_damage()
        }
    }
}
