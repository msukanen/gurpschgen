use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::misc::{costly::Costly, leveled::Leveled, mod_grouped::ModGrouped, named::Named};

static RX_ADQ: Lazy<Regex> = Lazy::new(||Regex::new(r"^\s*((?<c1>[-+]?\d+)\s*/\s*(?<c2>[-+]?\d+)|(?<c3>[-]?\d+))(?:\s*;\s*(?:(?<maxlvl>\d+)?(?:\s*;\s*(?:(?<bonus>[^;]*)(?:\s*;\s*(?:(?<given>[^;]*)(?:;\s*(?<modgr>[^;]*)?)?)?)?)?)?)?)?").unwrap());

/**
 Container for advantages, disadvantages and quirks.
 */
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Adq {
    name: String,
    initial_cost: i32,
    cost_increment: i32,
    level: usize,
    max_level: usize,
    bonus_mods: Vec<String>,
    given: Vec<String>,
    mod_groups: Vec<String>,
}

impl Adq {
    /**
     Get initial purchasing point cost (a.k.a. cost of the 1st level/rank).

     **Returns** some value.
     */
    pub fn initial_cost(&self) -> i32 {
        self.initial_cost
    }

    /**
     Get per-level cost increment, which is applied after 1st level/rank for
     each additional level from there on.

     **Returns** some value.
     */
    pub fn cost_increment(&self) -> i32 {
        self.cost_increment
    }

    /**
     Get names of what the [Adq] gives with it.

     **Returns** a (possibly empty) vector of names of things this [Adq] gives along with it.
     */
    pub fn gives(&self) -> &Vec<String> {
        &self.given
    }

    /**
     Whatever these are&hellip; nobody knows.
     */
    pub fn bonus_mods(&self) -> &Vec<String> {
        &self.bonus_mods
    }
}

impl Named for Adq {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Costly for Adq {
    fn cost(&self) -> f64 {
        //TODO: initial establishment of cost calc.
        (match self.level {
            ..=0 => 0,
            1 => self.initial_cost,
            n => self.initial_cost + (n as i32 - 1) * self.cost_increment
        }) as f64
    }
}

impl Leveled for Adq {
    fn level(&self) -> usize {
        self.level
    }

    fn max_level(&self) -> Option<usize> {
        self.max_level.into()
    }
}

impl ModGrouped for Adq {
    fn mod_groups(&self) -> &Vec<String> {
        &self.mod_groups
    }
}

impl From<(&str, &str)> for Adq {
    /**
     Advantages and Disadvantages have the form:
     
     name;
     initial cost/cost increment; max no. LEVELS; bonus mods; given ads/disads/skills; modifier groups used
    */
    fn from(value: (&str, &str)) -> Self {
        let name = String::from(value.0);
        if let Some(caps) = RX_ADQ.captures(value.1) {
            let initial_cost;
            let mut cost_increment = 0;
            let mut max_level = 1;
            let mut bonus_mods = vec![];
            let mut given = vec![];
            let mut mod_groups = vec![];

            // Let's deal with (c1/c2)|(c3) regexes first.
            if let Some(cap) = caps.name("c1") {
                // Note that c1 & c2 capture at once and so we can just unwrap c2 instead of specifically checking for it.
                initial_cost = cap.as_str().parse::<i32>().unwrap();
                cost_increment = caps.name("c2").unwrap().as_str().parse::<i32>().unwrap();
            } else if let Some(cap) = caps.name("c3") {
                initial_cost = cap.as_str().parse::<i32>().unwrap();
            } else {
                panic!("FATAL: cost not defined in {:?}", value.1)
            }

            // Got max level defined?
            if let Some(cap) = caps.name("maxlvl") {
                max_level = cap.as_str().parse::<usize>().unwrap();
            }

            if let Some(cap) = caps.name("bonus") {
                for x in cap.as_str().split(",") {
                    let x = x.trim();
                    if !x.is_empty() {
                        bonus_mods.push(x.to_string())
                    }
                }
            }

            if let Some(cap) = caps.name("given") {
                for x in cap.as_str().split(",") {
                    let x = x.trim();
                    if !x.is_empty() {
                        given.push(x.trim().to_string())
                    }
                }
            }

            if let Some(cap) = caps.name("modgr") {
                for x in cap.as_str().split(",") {
                    let x = x.trim();
                    if !x.is_empty() {
                        mod_groups.push(x.trim().to_string())
                    }
                }
            }

            Adq {
                name,
                initial_cost,
                cost_increment,
                max_level,
                bonus_mods,
                given,
                mod_groups,
                level: 0,
            }
        } else {
            panic!("FATAL: malformed ADQ {:?} {:?}", value.0, value.1)
        }
    }
}

#[cfg(test)]
mod adq_tests {
    use crate::misc::{leveled::Leveled, named::Named};

    use super::Adq;

    #[test]
    fn adq_is_constructed_from_short_real_data() {
        let data = "10/5; 2";
        let adq = Adq::from(("Adq", data));
        assert_eq!("Adq", adq.name);
        assert_eq!(10, adq.initial_cost);
        assert_eq!(5, adq.cost_increment);
        assert_eq!(2, adq.max_level);
    }

    #[test]
    fn adq_is_constructed_from_partial_real_data() {
        let data = "10/5; 2;;Gluttony, Mohican";
        let adq = Adq::from(("Adq", data));
        assert_eq!("Adq", adq.name);
        assert_eq!(10, adq.initial_cost);
        assert_eq!(5, adq.cost_increment);
        assert_eq!(2, adq.max_level);
        assert_eq!(2, adq.given.len());
    }

    #[test]
    fn adq_is_constructed_from_full_real_data() {
        let data = "10/5; 2;;Gluttony, Mohican; Toxifiers, Motorists, Woke";
        let adq = Adq::from(("Adq", data));
        assert_eq!("Adq", adq.name());
        assert_eq!(10, adq.initial_cost());
        assert_eq!(5, adq.cost_increment());
        assert_eq!(2, if let Some(x) = adq.max_level() {x} else {panic!("max_level != 2")});
        assert_eq!(2, adq.given.len());
        assert_eq!(3, adq.mod_groups.len());
    }

    #[test]
    fn adq_is_constructed_from_mixed_and_extra_data() {
        let data = "10/5; 2;;, Mohican; Toxifiers, Motorists, Woke;Bongo";
        let adq = Adq::from(("Adq", data));
        assert_eq!("Adq", adq.name());
        assert_eq!(10, adq.initial_cost());
        assert_eq!(5, adq.cost_increment());
        assert_eq!(2, if let Some(x) = adq.max_level() {x} else {panic!("max_level != 2")});
        assert_eq!(1, adq.given.len());
        assert_eq!(3, adq.mod_groups.len());
    }
}
