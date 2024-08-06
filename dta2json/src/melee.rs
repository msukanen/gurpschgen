use gurpschgen_lib::{damage::DamageDelivery, equipment::weapon::melee::Melee};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{damage::damage_from_str, ranged::RX_R_ACC, weapon::{RX_DMGD, RX_MAX_DMG}, RX_COST_WEIGHT};

static RX_R_ST: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:ST\s*(?<st>\d+))").unwrap());

/**
 Construct a melee weapon from given `value`.

 **dev Note**: the weapon specs are too random in contents to parse with a simple [Regex].
 */
pub(crate) fn melee_wpn_from_tuple(value: (&str, &str)) -> Melee {
    let mut cost = None;
    let mut weight = None;
    let mut skill = None;
    let mut damage = vec![];
    let mut notes = None;
    let mut acc = None;
    let mut mod_groups = vec![];
    let mut st_req = None;
    let mut max_damage = None;
    for (index, x) in value.1.split(";").enumerate() {
        match index {
            0 => for d in x.split(",") {
                let d = d.trim();
                if let Some(x) = RX_R_ACC.captures(d) {
                    acc = x.name("acc").unwrap().as_str().parse::<i32>().unwrap().into()
                }
                else if let Some(x) = RX_R_ST.captures(d) {
                    st_req = x.name("st").unwrap().as_str().parse::<i32>().unwrap().into()
                }
                else if let Some(_) = RX_DMGD.captures(d) {
                    damage.push(damage_from_str(d.trim()))
                }
                else if let Some(x) = RX_MAX_DMG.captures(d) {
                    max_damage = Some(DamageDelivery::Dice(
                        x.name("dmgd").unwrap().as_str().parse::<i32>().unwrap(),
                        if let Some(x) = x.name("dmgb") {
                            x.as_str().parse::<i32>().unwrap()
                        } else {0}))
                }
            },
            1 => if let Some(cap) = RX_COST_WEIGHT.captures(x) {
                if let Some(c) = cap.name("cost") {
                    cost = c.as_str().trim().parse::<f64>().unwrap().into()
                }

                if let Some(c) = cap.name("wt") {
                    weight = c.as_str().trim().parse::<f64>().unwrap().into()
                }
            },
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

    Melee { name: value.0.trim().to_string(), damage, cost, weight, skill, notes, mod_groups, acc, st_req, max_damage }
}

#[cfg(test)]
mod melee_tests {
    use gurpschgen_lib::{damage::{Damage, DamageDelivery}, equipment::weapon::Weapon, misc::{costly::Costly, damaged::Damaged, noted::Noted, skilled::Skilled, weighed::Weighed}};

    use crate::{melee::melee_wpn_from_tuple, weapon::wpn_from_tuple};

    #[test]
    fn crafting_melee_weapon_works() {
        let data = ("        Broadsword  ", "   Cut/Sw+1, Cr/Thr+1, Imp/Sw+3, Cut/Thr-2;  500,3.0  ;  Broadsword ;  It's absolutely horrible...; Sword Quality, Weapon, Melee Weapon");
        let wpn = melee_wpn_from_tuple(data);
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
        let wpn = melee_wpn_from_tuple(data);
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
        let wpn = wpn_from_tuple(data);
        assert!(match wpn {
            Weapon::Melee(_) => true,
            _ => false
        });
        assert_eq!(DamageDelivery::Dice(1, 2), wpn.max_damage().clone().unwrap());
    }
}
