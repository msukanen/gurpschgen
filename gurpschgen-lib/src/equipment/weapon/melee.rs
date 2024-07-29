use regex::Regex;

use crate::{damage::Damage, equipment::weapon::{RX_DMGD, ranged::RX_R_ACC}, misc::{costly::Costly, noted::Noted, skilled::Skilled, st_req::STRequired, weighed::Weighed}, RX_COST_WEIGHT};

thread_local! {
    static RX_R_ST: Regex = Regex::new(r"(?:ST\s*(?<st>\d+))").unwrap();
}

/**
 Melee weapon data.
 */
#[derive(Debug, Clone)]
pub struct Melee {
    name: String,
    damage: Vec<Damage>,
    cost: Option<f64>,
    weight: Option<f64>,
    skill: Option<String>,
    notes: Option<String>,
    mod_groups: Vec<String>,
    acc: Option<i32>,
    st_req: Option<i32>,
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

impl From<(&str, &str)> for Melee {
    /**
     Construct a melee weapon from given `value`.

     **dev Note**: the weapon specs are too random in contents to parse with a simple [Regex].
     */
    fn from(value: (&str, &str)) -> Self {
        let mut cost = None;
        let mut weight = None;
        let mut skill = None;
        let mut damage = vec![];
        let mut notes = None;
        let mut acc = None;
        let mut mod_groups = vec![];
        let mut st_req = None;
        for (index, x) in value.1.split(";").enumerate() {
            match index {
                0 => for d in x.split(",") {
                    let d = d.trim();
                    if let Some(x) = RX_R_ACC.with(|rx| rx.captures(d)) {
                        acc = x.name("acc").unwrap().as_str().parse::<i32>().unwrap().into()
                    } else if let Some(x) = RX_R_ST.with(|rx| rx.captures(d)) {
                        st_req = x.name("st").unwrap().as_str().parse::<i32>().unwrap().into()
                    } else if let Some(x) = RX_DMGD.with(|rx| rx.captures(d)) {
                        //println!("ERR? {:?}---{:?}",value.0,value.1);
                        damage.push(Damage::from(d.trim()))
                    }
                },
                1 => RX_COST_WEIGHT.with(|rx| if let Some(cap) = rx.captures(x) {
                    if let Some(c) = cap.name("cost") {
                        cost = c.as_str().trim().parse::<f64>().unwrap().into()
                    }

                    if let Some(c) = cap.name("wt") {
                        weight = c.as_str().trim().parse::<f64>().unwrap().into()
                    }
                }),
                2 => {
                    let x = x.trim();
                    if !x.is_empty() {
                        skill = x.to_string().into()
                    }
                },
                3 => {
                    let x = x.trim();
                    if !x.is_empty() {
                        notes = x.to_string().into()
                    }
                },
                4 => {
                    for x in x.split(",") {
                        let x = x.trim();
                        if !x.is_empty() {
                            mod_groups.push(x.to_string())
                        }
                    }
                },
                5 => (),// This is usually caused by stray semicolon(s), so no need to *quite* panic ;-)
                _ => panic!("FATAL: extraneous semicolons (at end of) \"{}\"", value.1)// But here we can panic!
            }
        }

        Self { name: value.0.trim().to_string(), damage, cost, weight, skill, notes, mod_groups, acc, st_req }
    }
}

#[cfg(test)]
mod melee_tests {
    use crate::{damage::{Damage, DamageDelivery}, equipment::weapon::Weapon, misc::{costly::Costly, noted::Noted, skilled::Skilled, weighed::Weighed}};

    use super::Melee;

    #[test]
    fn crafting_melee_weapon_works() {
        let data = ("        Broadsword  ", "   Cut/Sw+1, Cr/Thr+1, Imp/Sw+3, Cut/Thr-2;  500,3.0  ;  Broadsword ;  It's absolutely horrible...; Sword Quality, Weapon, Melee Weapon");
        let wpn = Melee::from(data);
        assert_eq!("Broadsword", wpn.name);
        assert_eq!(500.0, wpn.cost());
        assert_eq!(Some(3.0), wpn.weight());
        assert_eq!(Some("Broadsword"), wpn.skill());
        assert_eq!(Some("It's absolutely horrible..."), wpn.notes());
        assert_eq!(3, wpn.mod_groups.len());
        assert!(wpn.damage.contains(&Damage::Cut(DamageDelivery::Sw(1))));
        assert!(wpn.damage.contains(&Damage::Cr(DamageDelivery::Thr(1))));
        assert!(wpn.damage.contains(&Damage::Imp(DamageDelivery::Sw(3))));
        assert!(wpn.damage.contains(&Damage::Cut(DamageDelivery::Thr(-2))));
    }

    #[test]
    fn minimalistic_approach_works() {
        let data = ("        Broadsword  ", "   Cut/Sw+1,;  ;  ;  It's absolutely horrible...; ;");
        let wpn = Melee::from(data);
        assert_eq!("Broadsword", wpn.name);
        assert_eq!(0.0, wpn.cost());
        assert_eq!(None, wpn.weight());
        assert_eq!(None, wpn.skill());
        assert_eq!(Some("It's absolutely horrible..."), wpn.notes());
        assert_eq!(0, wpn.mod_groups.len());
        assert!(wpn.damage.contains(&Damage::Cut(DamageDelivery::Sw(1))));
        assert!(!wpn.damage.contains(&Damage::Cr(DamageDelivery::Sw(1))));
    }

    #[test]
    fn max_dmg_works() {
        let data = ("Punal (knife)", "Cut/Sw-2, Imp/Thr, Maximum damage 1d+2;40,1.0;Knife;;Sword Quality, Weapon, Melee Weapon");
        let wpn = Weapon::from(data);
        assert!(match wpn {
            Weapon::Melee(_) => true,
            _ => false
        })
    }
}
