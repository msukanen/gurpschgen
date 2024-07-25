use std::collections::HashSet;

use regex::Regex;

use crate::{damage::{DamageResistance, PassiveDefense}, misc::{costly::Costly, mod_grouped::ModGrouped, named::Named, weighed::Weighed}, RX_COST_WEIGHT};

thread_local! {
    //  Steel Skullcap (improved); PD2,DR3,Covers:3-4; 20, 2.0000; ; ; Armor: Head
    static RX_PD: Regex = Regex::new(r"(?:\s*PD\s*(?<pd>\d+))").unwrap();
    static RX_DR: Regex = Regex::new(r"(?:\s*DR\s*(?<dr>\d+))").unwrap();
    static RX_COVER: Regex = Regex::new(r"(?:\s*Covers:(?<cover>[-,\d\s]+))").unwrap();
}

#[derive(Debug, Clone)]
pub struct Armor {
    name: String,
    dr: Option<DamageResistance>,
    pd: Option<PassiveDefense>,
    cover: HashSet<i32>,
    cost: Option<f64>,
    weight: Option<f64>,// most things have weight, but e.g. magic armor wt. might be neglible
    mod_groups: Vec<String>,
}

impl Costly for Armor {
    fn cost(&self) -> f64 {
        if let Some(x) = self.cost {x} else {0.0}
    }
}

impl Weighed for Armor {
    fn weight(&self) -> Option<f64> {
        self.weight
    }
}

impl Named for Armor {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Armor {
    /**
     Get the armor's DR ([damage resistance][DamageResistance]), if applicable.
     */
    pub fn dr(&self) -> &Option<DamageResistance> {
        &self.dr
    }

    /**
     Get the armor's PD ([passive defense][PassiveDefense]), if applicable.
     */
    pub fn pd(&self) -> &Option<PassiveDefense> {
        &self.pd
    }

    /**
     Get hit locations covered.

     **Returns** a (possibly empty) hash of covered hit locations.
     */
    pub fn cover(&self) -> &HashSet<i32> {
        &self.cover
    }
}

impl ModGrouped for Armor {
    fn mod_groups(&self) -> &Vec<String> {
        &self.mod_groups
    }
}

impl From<(&str, &str)> for Armor {
    /**
     Construct [Armor] from (a complex) `value`.
     */
    fn from(value: (&str, &str)) -> Self {
        let mut pd = None;
        let mut dr = None;
        let mut cover = HashSet::new();
        let mut cost = None;
        let mut weight = None;
        let mut mod_groups = vec![];
        for (index, x) in value.1.split(";").enumerate() {
            match index {
                // specs
                0 => {
                    if let Some(x) = RX_PD.with(|rx| rx.captures(x)) {
                        pd = PassiveDefense::from(x.name("pd").unwrap().as_str().parse::<i32>().unwrap()).into()
                    }
                    if let Some(x) = RX_DR.with(|rx| rx.captures(x)) {
                        dr = DamageResistance::from(x.name("dr").unwrap().as_str().parse::<i32>().unwrap()).into()
                    }
                    // cover is e.g. "3-4, 6, 11-15"
                    if let Some(x) = RX_COVER.with(|rx| rx.captures(x)) {
                        let parts = x.name("cover").unwrap().as_str().split(",");
                        for p in parts {
                            for c in p.trim().split("-") {
                                cover.insert(c/*.trim()*/.parse::<i32>().unwrap());
                            }
                        }
                    }
                },
                // cost, weight
                1 => if let Some(x) = RX_COST_WEIGHT.with(|rx| rx.captures(x)) {
                    if let Some(x) = x.name("cost") {
                        cost = x.as_str().parse::<f64>().unwrap().into()
                    }
                    if let Some(x) = x.name("wt") {
                        weight = x.as_str().parse::<f64>().unwrap().into()
                    }
                } else {
                    panic!("FATAL: no cost and/or weight defined in {:?}", value)
                },
                // modgr
                4 => for x in x.split(",") {
                    mod_groups.push(x.to_string())
                },
                _ => ()
            }
        }

        Self {
            name: value.0.trim().to_string(),
            dr, pd, cover, cost, weight, mod_groups,
        }
    }
}

#[cfg(test)]
mod armor_tests {
    use crate::misc::costly::Costly;

    use super::Armor;

    #[test]
    fn full_armor_works() {
        let data = "PD 0,    DR1  ,Covers:6  ,8-14   ,17-18;  50 , 1.00 ; ; ;   Armor: Clothing, Clothing Quality, Race Sizing ;;;";
        let armor = Armor::from(("Dress", data));
        
        assert_eq!("Dress", armor.name);
        
        let Some(pd) = &armor.pd else {panic!("malformed data or regex: {data}")};
        assert_eq!(0, pd.value());
        
        let Some(dr) = &armor.dr else {panic!("malformed data or regex: {data}")};
        assert_eq!(1, dr.value());
        
        assert_eq!(50.0, armor.cost());
        
        let Some(wt) = &armor.weight else {panic!("malformed data or regex: {data}")};
        assert_eq!(1.0, *wt);

        assert_eq!(3, armor.mod_groups.len());
    }

    #[test]
    fn shield_works() {
        let data = "PD 2;  50 , 1.00 ; ; ;   Shield Construction ;;;";
        let armor = Armor::from(("Buckler", data));
        
        assert_eq!("Buckler", armor.name);
        
        let Some(pd) = &armor.pd else {panic!("malformed data or regex: {data}")};
        assert_eq!(2, pd.value());

        if let Some(dr) = &armor.dr {
            assert_eq!(0, dr.value())
        }
        
        let Some(wt) = &armor.weight else {panic!("malformed data or regex: {data}")};
        assert_eq!(1.0, *wt);

        assert_eq!(1, armor.mod_groups.len());
    }
}
