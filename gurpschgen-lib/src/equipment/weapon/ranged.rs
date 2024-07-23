pub mod rof;

use rof::RoF;

use regex::Regex;

use crate::{RX_COST_WEIGHT, RX_DMGD, damage::Damage, misc::{costly::Costly, noted::Noted, skilled::Skilled, weighed::Weighed}};

thread_local! {
    static RX_R_SS: Regex = Regex::new(r"(?:\s*SS\s*(?<ss>[-+]?\d+))").unwrap();
    pub(crate) static RX_R_ACC: Regex = Regex::new(r"(?:\s*[aA]cc\s*(?<acc>[-+]?\d+))").unwrap();
    static RX_R_ROF: Regex = Regex::new(r"(?:\s*[rR][oO][fF]\s+(?<rof>(?<rof1>\d+)(?:~|\/(?<rof2>\d+))))").unwrap();
    static RX_R_RCL: Regex = Regex::new(r"(?:\s*[rR]cl\s*(?<rcl>[-+]?\d+))").unwrap();
    static RX_R_HDMG: Regex = Regex::new(r"(?:\s*1\/2D?\s+(?<hdmg>\d+))").unwrap();
    static RX_R_MIN: Regex = Regex::new(r"(?:\s*(?:min|Min|MIN)\s+(?<min>\d+))").unwrap();
    static RX_R_MAX: Regex = Regex::new(r"(?:\s*(?:max|Max|MAX)\s+(?<max>\d+))").unwrap();
    static RX_R_SHOTS: Regex = Regex::new(r"(?:\s*(?:[sS]hots\s+(?<shots>\d+)))").unwrap();
}

/**
 Ranged weapon data.
 */
#[derive(Debug, Clone)]
pub struct Ranged {
    name: String,
    damage: Vec<Damage>,
    acc: i32,
    ss: i32,
    rof: Option<RoF>,// RoF does not apply to thrown weapons...
    rcl: Option<i32>,// some weapons have recoil, some don't.
    min_range: Option<i32>,// some weapons cannot be fired to/at any closer range (at least not safely...).
    half_dmg_range: Option<i32>,// some weapons don't lose dmg over distance...
    max_range: Option<i32>,// everything has some sort of "effective max range", but for some this depends on external factors (e.g. ST in case of bows).
    cost: Option<f64>,
    weight: Option<f64>,
    skill: Option<String>,
    notes: Option<String>,
    shots: Option<i32>,// in case the weapon has a magazine or somesuch...
    mod_groups: Vec<String>,
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
    fn skill(&self) -> Option<&str> {
        if let Some(x) = &self.skill {
            x.as_str().into()
        } else {
            None
        }
    }
}

impl From<(&str, &str)> for Ranged {
    /**
     Construct a ranged weapon from given `value`.

     **dev Note**: the weapon specs are too random in contents to parse with a simple [Regex].
     */
    fn from(value: (&str, &str)) -> Self {
        let mut cost = None;
        let mut weight = None;
        let mut skill = None;
        let mut damage = vec![];
        let mut notes = None;
        let mut mod_groups = vec![];
        let mut ss = 0;
        let mut acc = 0;
        let mut rof = None;
        let mut rcl = None;
        let mut half_dmg_range = None;
        let mut max_range = None;
        let mut min_range = None;
        let mut shots = None;
        for (index, x) in value.1.split(";").enumerate() {
            match index {
                0 => for d in x.split(",") {
                    let d = d.trim();
                    if let Some(x) = RX_DMGD.with(|rx| rx.captures(d)) {// TODO: this unfortunately will get repeated in Damage::from(). Fix somehow?
                        damage.push(Damage::from(x.get(0).unwrap().as_str()))
                    } else if let Some(x) = RX_R_ACC.with(|rx| rx.captures(d)) {
                        acc = x.name("acc").unwrap().as_str().parse::<i32>().unwrap()
                    } else if let Some(x) = RX_R_SS.with(|rx| rx.captures(d)) {
                        ss = x.name("ss").unwrap().as_str().parse::<i32>().unwrap()
                    } else if let Some(x) = RX_R_ROF.with(|rx| rx.captures(d)) {
                        rof = RoF::from(x).into()
                    } else if let Some(x) = RX_R_RCL.with(|rx| rx.captures(d)) {
                        rcl = x.name("rcl").unwrap().as_str().parse::<i32>().unwrap().into()
                    } else if let Some(x) = RX_R_HDMG.with(|rx| rx.captures(d)) {
                        half_dmg_range = x.name("hdmg").unwrap().as_str().parse::<i32>().unwrap().into()
                    } else if let Some(x) = RX_R_MAX.with(|rx| rx.captures(d)) {
                        max_range = x.name("max").unwrap().as_str().parse::<i32>().unwrap().into()
                    } else if let Some(x) = RX_R_SHOTS.with(|rx| rx.captures(d)) {
                        shots = x.name("shots").unwrap().as_str().parse::<i32>().unwrap().into()
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

        Self { name: value.0.trim().to_string(), damage, cost, weight, skill, notes, mod_groups, ss, acc, rof, rcl, min_range, half_dmg_range, max_range, shots }
    }
}

#[cfg(test)]
mod ranged_tests {
    use crate::damage::{Damage, DamageDelivery};

    use super::Ranged;

    #[test]
    fn ranged_1_works() {
        let data = ("  AT-3 Sagger (ATGM)  ", "Cr/48+0, Acc+14, SS 23, Min 300, 1/2D n/a, Max 3000, RoF 1/10, Shots 1; 20000,260");
        let rng = Ranged::from(data);
        assert_eq!("AT-3 Sagger (ATGM)", rng.name);
        assert!(rng.damage.contains(&Damage::Cr(DamageDelivery::Dice(48, 0))));
    }

    #[test]
    fn ranged_2_works() {
        let data = ("  IMI Eagle .50AE", "Cr/3+2(X1.5), Acc+3, RoF 3~, ST 13, Rcl-4, Shots 9+1; 1000,4.5; Guns: Pistol");
        let rng = Ranged::from(data);
        assert_eq!("IMI Eagle .50AE", rng.name);
    }
}
