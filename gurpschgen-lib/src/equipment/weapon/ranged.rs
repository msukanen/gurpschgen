pub mod rof;
pub mod shots;

use rof::RoF;

use serde::{Deserialize, Serialize};
use shots::Shots;

use crate::{damage::{Damage, DamageDelivery}, misc::{costly::Costly, damaged::Damaged, mod_grouped::ModGrouped, noted::Noted, skilled::Skilled, st_req::STRequired, weighed::Weighed}};

/**
 Ranged weapon data.
 */
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ranged {
    pub name: String,
    pub damage: Vec<Damage>,
    pub max_damage: Option<DamageDelivery>,
    pub acc: i32,
    pub ss: Option<i32>,
    pub rof: Option<RoF>,
    pub rcl: Option<i32>,
    pub min_range: Option<i32>,
    pub half_dmg_range: Option<i32>,
    pub max_range: Option<i32>,
    pub st_req: Option<i32>,
    pub tripod: bool,
    pub cost: Option<f64>,
    pub weight: Option<f64>,
    pub skill: Option<String>,
    pub notes: Option<String>,
    pub shots: Option<Shots>,
    pub mod_groups: Vec<String>,
    pub rl_year: Option<i32>,
    pub rl_country: Option<String>,
    pub tl: Option<i32>,
    pub lc: Option<i32>,
}

impl Costly for Ranged {
    fn cost(&self) -> f64 {
        match self.cost {
            Some(x) => x,
            _ => 0.0
        }
    }
}

impl Noted for Ranged {
    fn notes(&self) -> Option<&str> {
        if let Some(x) = &self.notes {
            x.as_str().into()
        } else {
            None
        }
    }
}

impl Weighed for Ranged {
    fn weight(&self) -> Option<f64> {
        self.weight
    }
}

impl Skilled for Ranged {
    /**
     The skill required to operate the weapon.
    
     For some there's no "skill" beyond e.g. assigning target with a computer, in which case `None` suffices.
     */
    fn skill(&self) -> Option<&str> {
        if let Some(x) = &self.skill {
            x.as_str().into()
        } else {
            None
        }
    }
}

impl STRequired for Ranged {
    fn st_req(&self) -> &Option<i32> {
        &self.st_req
    }
}

impl Damaged for Ranged {
    fn damage(&self) -> &Vec<Damage> {
        &self.damage
    }

    fn max_damage(&self) -> &Option<DamageDelivery> {
        &self.max_damage
    }
}

impl Ranged {
    /// Name of the weapon.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Weapon's damage type(s).
    pub fn damage(&self) -> &Vec<Damage> {
        &self.damage
    }

    /// Weapon's accuracy, Acc.
    pub fn acc(&self) -> i32 {
        self.acc
    }
    
    /// Weapon's snap shot, SS, if applicable.
    pub fn ss(&self) -> &Option<i32> {
        &self.ss
    }
    
    /// Weapon's RoF, if applicable.
    pub fn rof(&self) -> &Option<RoF> {// RoF does not apply to thrown weapons...
        &self.rof
    }
    
    /// Weapon's recoil, Rcl, if applicable.
    pub fn rcl(&self) -> &Option<i32> {// some weapons have recoil, some don't.
        &self.rcl
    }
    
    /// Weapon's minimum range to fire, if applicable. Generally for rocket/grenade launchers, etc.
    pub fn min_range(&self) -> &Option<i32> {// some weapons cannot be fired to/at any closer range (at least not safely...).
        &self.min_range
    }
    
    /// Weapon's half-damage range, if applicable. Most self-propelled munition carriers don't care.
    pub fn half_dmg_range(&self) -> &Option<i32> {// some weapons don't lose dmg over distance...
        &self.half_dmg_range
    }
    
    /// Weapon's max-range. Past this the weapon doesn't either do damage or the munition can't fly any further.
    pub fn max_range(&self) -> &Option<i32> {// everything has some sort of "effective max range", but for some this depends on external factors (e.g. ST in case of bows).
        &self.max_range
    }
    
    /// Minimum ST required to operate properly, if applicable.
    pub fn st_req(&self) -> &Option<i32> {
        &self.st_req
    }
    
    /// Weapon's self-carried ammunition amount, if applicable.
    pub fn shots(&self) -> &Option<Shots> {
        &self.shots
    }
}

impl ModGrouped for Ranged {
    /// Modifiers which affect the weapon. E.g., quality, extra modules, etc.
    fn mod_groups(&self) -> &Vec<String> {
        &self.mod_groups
    }
}
