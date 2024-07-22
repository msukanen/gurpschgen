use crate::{RX_ARMOR, damage::{DamageResistance, PassiveDefense}, misc::costly::Costly};

#[derive(Debug, Clone)]
pub struct Armor {
    name: String,
    dr: Option<DamageResistance>,
    pd: Option<PassiveDefense>,
    cost: Option<f64>,
    weight: Option<f64>,// most things have weight, but e.g. magic armor wt. might be neglible
    mod_groups: Vec<String>,
}

impl Costly for Armor {
    fn cost(&self) -> f64 {
        if let Some(x) = self.cost {x} else {0.0}
    }
}

impl From<(&str, &str)> for Armor {
    fn from(value: (&str, &str)) -> Self {
        RX_ARMOR.with(|rx| if let Some(caps) = rx.captures(value.1) {
            let cost = if let Some(cap) = caps.name("cost") {
                Some(cap.as_str().parse::<f64>().unwrap())
            } else {
                None
            };
            
            let dr = if let Some(cap) = caps.name("dr") {
                Some(DamageResistance::All(cap.as_str().parse::<i32>().unwrap()))
            } else {
                todo!("FATAL: armor w/o DR?")
            };

            let pd = if let Some(cap) = caps.name("pd") {
                Some(PassiveDefense::All(cap.as_str().parse::<i32>().unwrap()))
            } else {
                todo!("FATAL: armor w/o PD?")
            };

            let wt = if let Some(cap) = caps.name("wt") {
                Some(cap.as_str().parse::<f64>().unwrap())
            } else {
                None
            };

            let mut mod_groups = vec![];
            if let Some(cap) = caps.name("modgr") {
                for x in cap.as_str().split(",") {
                    let x = x.trim();
                    if !x.is_empty() {
                        mod_groups.push(x.to_string())
                    }
                }
            }

            Self {
                name: value.0.to_string(),
                cost,
                weight: wt,
                dr, pd,
                mod_groups,
            }
        } else {
            panic!("FATAL: ill formed armor \"{}\"", value.1)
        })
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
        
        if let Some(pd) = &armor.pd {
            assert_eq!(0, pd.value());
        } else {
            panic!("malformed data or regex: {data}")
        }
        
        if let Some(dr) = &armor.dr {
            assert_eq!(1, dr.value());
        } else {
            panic!("malformed data or regex: {data}")
        }
        
        assert_eq!(50.0, armor.cost());
        
        if let Some(wt) = &armor.weight {
            assert_eq!(1.0, *wt);
        } else {
            panic!("malformed data or regex: {data}")
        }

        assert_eq!(3, armor.mod_groups.len());
    }
}
