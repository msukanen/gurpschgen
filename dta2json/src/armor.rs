use std::collections::HashSet;

use gurpschgen_lib::{damage::{DamageResistance, PassiveDefense}, equipment::armor::Armor};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{container::container_from_captures, item::RX_WT, stat::stat_from_str, RX_COST_WEIGHT};

pub(crate) static RX_IS_ARMOR: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:(?:PD|DR)\s*\d)").unwrap());

/**
 Construct [Armor] from (a complex) `value`.
 */
pub(crate) fn armor_from_tuple(value: (&str, &str)) -> Armor {
    static RX_PD: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:\s*PD\s*(?<pd>\d+))").unwrap());
    static RX_DR: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:\s*DR\s*(?<dr>\d+))").unwrap());
    static RX_COVER: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:\s*[cC]overs(?::\s*|\s+)(?<cover>(\d+-\d+|[,\s]|\d+)+))").unwrap());
    static RX_STAT: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:(?<val>[-+]\d+)\s*(?<what>DX|HT|IQ|ST))").unwrap());
    static RX_SK_AFF: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:(?<val>[-+]\d+)\s+(?<what>.*))").unwrap());
    static RX_EXTRA: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:Face protection)").unwrap());

    let mut pd = None;
    let mut dr = None;
    let mut cover = HashSet::new();
    let mut cost = None;
    let mut weight = None;
    let mut mod_groups = vec![];
    let mut skill = None;
    let mut stats_affected = vec![];
    let mut container = None;
    let mut skills_affected = vec![];
    let mut _extra = vec![];

    for (index, x) in value.1.split(";").enumerate() {
        let mut x = x.trim().to_string();
        match index {
            // specs
            0 => {
                // cover is e.g. "3-4, 6, 11-15"
                if let Some(caps) = RX_COVER.captures(x.as_str()) {
                    let parts = caps.name("cover").unwrap().as_str().split(",");
                    for p in parts {
                        let p = p.trim();
                        if p.is_empty() {
                            continue;
                        }
                        for c in p.split("-") {
                            cover.insert(c.parse::<i32>().unwrap());
                        }
                    }
                    x = x.replace(caps.get(0).unwrap().as_str(), "");
                }

                for x in x.split(",") {
                    let x = x.trim();
                    if x.is_empty() { continue; }

                    if let Some(x) = RX_PD.captures(x) {
                        pd = PassiveDefense::from(x.name("pd").unwrap().as_str().parse::<i32>().unwrap()).into()
                    } else if let Some(x) = RX_DR.captures(x) {
                        dr = DamageResistance::from(x.name("dr").unwrap().as_str().parse::<i32>().unwrap()).into()
                    } else if let Some(x) = RX_STAT.captures(x) {
                        stats_affected.push((
                            stat_from_str(x.name("what").unwrap().as_str().trim()),
                            x.name("val").unwrap().as_str().parse::<i32>().unwrap()
                        ))
                    } else if let Some(x) = RX_WT.captures(x) {
                        container = container_from_captures(x).into()
                    } else if let Some(x) = RX_SK_AFF.captures(x) {
                        skills_affected.push((
                            x.name("what").unwrap().as_str().trim().to_string(),
                            x.name("val").unwrap().as_str().parse::<i32>().unwrap()
                        ))
                    } else if let Some(_) = RX_EXTRA.captures(x) {
                        _extra.push(x.trim().to_string())
                    } else if x.starts_with("3") || x.trim().eq("Covers:") {
                        /* no op */
                    } else {
                        todo!("{x} --?!")
                    }
                }
            },
            // cost, weight
            1 => if let Some(x) = RX_COST_WEIGHT.captures(x.as_str()) {
                if let Some(x) = x.name("cost") {
                    cost = x.as_str().parse::<f64>().unwrap().into()
                }
                if let Some(x) = x.name("wt") {
                    weight = x.as_str().parse::<f64>().unwrap().into()
                }
            } else {
                panic!("FATAL: no cost and/or weight defined in {:?}", value)
            },
            // skill to use, if any
            2 => if !x.is_empty() {
                skill = x.to_string().into()
            },
            // modgr
            4 => for x in x.split(",") {
                mod_groups.push(x.to_string())
            },
            3|5 => if !x.is_empty() {
                todo!("3|5 → \"{x}\" ?!")
            }
            _ => ()
        }
    }

    Armor {_extra,
        name: value.0.trim().to_string(), skill,
        dr, pd, cover, cost, weight, mod_groups,
        stats_affected, container, skills_affected,
    }
}

#[cfg(test)]
mod armor_tests {
    use gurpschgen_lib::{equipment::item::container::Container, misc::costly::Costly, skill::Stat};

    use crate::armor::armor_from_tuple;

    #[test]
    fn full_armor_works() {
        let data = "PD 0,    DR1  ,Covers:6  ,8-14   ,17-18, -1 DX;  50 , 1.00 ; ; ;   Armor: Clothing, Clothing Quality, Race Sizing ;;;";
        let armor = armor_from_tuple(("Dress", data));
        
        assert_eq!("Dress", armor.name);
        
        let Some(pd) = &armor.pd else {panic!("malformed data or regex: {data}")};
        assert_eq!(0, pd.value());
        
        let Some(dr) = &armor.dr else {panic!("malformed data or regex: {data}")};
        assert_eq!(1, dr.value());
        
        assert_eq!(50.0, armor.cost());
        
        let Some(wt) = &armor.weight else {panic!("malformed data or regex: {data}")};
        assert_eq!(1.0, *wt);

        assert_eq!(3, armor.mod_groups.len());

        assert_eq!(vec![(Stat::DX, -1)], armor.stats_affected)
    }

    #[test]
    fn shield_works() {
        let value = ("Large shield", "PD4, -2 weapon skill, -1 parry;90,25.0;Shield;;Shield, Armor");
        let sh = armor_from_tuple(value);
        
        assert_eq!("Large shield", sh.name);
        
        let Some(pd) = &sh.pd else {panic!("malformed data or regex: {:?}", value)};
        assert_eq!(4, pd.value());

        if let Some(dr) = &sh.dr {
            assert_eq!(0, dr.value())
        }
        
        let Some(wt) = &sh.weight else {panic!("malformed data or regex: {:?}", value)};
        assert_eq!(25.0, *wt);

        assert_eq!(2, sh.mod_groups.len());
        assert_eq!(vec![("weapon skill".to_string(), -2), ("parry".to_string(), -1)], sh.skills_affected)
    }

    #[test]
    fn container_works() {
        let value = ("Pack: small", "PD2,DR2,Covers:9-11,holds 40 lb., 3'x2'x1';60,3.0;");
        let c = armor_from_tuple(value);
        assert_eq!(Some(Container::Wt(40)), c.container);
    }
}
