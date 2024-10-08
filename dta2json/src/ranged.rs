use gurpschgen_lib::{damage::DamageDelivery, equipment::weapon::ranged::Ranged};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{damage::damage_from_str, equipment::{RX_COUNTRY, RX_LEGALITY, RX_TL}, rof::rof_from_captures, shots::shots_from_captures, weapon::{RX_DMGD, RX_MAX_DMG}, RX_COST_WEIGHT};

static RX_R_SS: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:SS\s*(?<ss>[-+]?\d+))").unwrap());
pub(crate) static RX_R_ACC: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:\s*[aA]cc\s*(?<acc>[-+]?\d+)?)").unwrap());
pub(crate) static RX_R_ROF: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:[rR][oO][fF]\s+(?<rof>(?:(?<rof1>\d+)|[sS]kill)(?:[*]|~|\/(?<rof2>\d+))?))").unwrap());
static RX_R_RCL: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:[rR]cl\s*(?<rcl>[-+]?\d+))").unwrap());
// RX_R_HDMG will ignore all non-numeric 1/2 entries:
static RX_R_HDMG: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:1\/2D?\s+(?:[nN]\/[aA]|(?<hdmg>\d+)))").unwrap());
static RX_R_MIN: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:(?:min|Min|MIN)\s+(?<min>\d+))").unwrap());
static RX_R_MAX: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:(?:max|Max|MAX)\s+(?:rnd[.]\s+)?(?<max>\d+))").unwrap());
pub(crate) static RX_R_SHOTS: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:[sS]hots\s+(?:(?:(?<battch>\d+)\/(?<batt>(?:A){1,3}|B|C|D|E|F))|(?:[(](?<fthrow1>\d+)[)])(?<fthrow2>\d+)|(?<xxxbelt>xxxB)|(?:(?<bfed>\d+)B(?<boxfed>ox)?)|(?:(?<splus>\d+)(?<splusmod>[+]\d+)?)))").unwrap());
static RX_R_ST: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:ST\s*(?:(?<st>\d+)|XX\([tT]ripod\)))").unwrap());
pub(crate) static RX_R_SPEC_DMG: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:[sS]pec)(?:\/(?<specvar>\d+))?").unwrap());
static RX_19XX: Lazy<Regex> = Lazy::new(||Regex::new(r"19\d\d").unwrap());

/**
 Construct a ranged weapon from given `value`.

 **dev Note**: the weapon specs are too random in contents to parse with a simple [Regex].
 */
pub(crate) fn ranged_wpn_from_tuple(value: (&str, &str)) -> Ranged {
    let mut cost = None;
    let mut weight = None;
    let mut skill = None;
    let mut damage = vec![];
    let mut notes = None;
    let mut mod_groups = vec![];
    let mut ss = None;
    let mut acc = 0;
    let mut rof = None;
    let mut rcl = None;
    let mut half_dmg_range = None;
    let mut max_range = None;
    let mut min_range = None;
    let mut shots = None;
    let mut st_req = None;
    let mut max_damage = None;
    let mut rl_year = None;
    let mut rl_country = None;
    let mut tripod = false;
    let mut tl = None;
    let mut lc = None;
    for (index, x) in value.1.split(";").enumerate() {
        match index {
            0 => for d in x.split(",") {
                let d = d.trim();
                if let Some(x) = RX_R_SPEC_DMG.captures(d) {
                    damage.push(damage_from_str(x.get(0).unwrap().as_str()))
                } else if let Some(x) = RX_DMGD.captures(d) {// TODO: this unfortunately will get repeated in Damage::from(). Fix somehow?
                    damage.push(damage_from_str(x.get(0).unwrap().as_str()))
                } else if let Some(x) = RX_R_ACC.captures(d) {
                    acc = if let Some(x) = x.name("acc") {
                        x.as_str().parse::<i32>().unwrap()
                    } else {0};
                } else if let Some(x) = RX_R_SS.captures(d) {
                    ss = x.name("ss").unwrap().as_str().parse::<i32>().unwrap().into()
                } else if let Some(x) = RX_R_ROF.captures(d) {
                    rof = rof_from_captures(x).into()
                } else if let Some(x) = RX_R_RCL.captures(d) {
                    rcl = x.name("rcl").unwrap().as_str().parse::<i32>().unwrap().into()
                } else if let Some(x) = RX_R_HDMG.captures(d) {
                    if let Some(x) = x.name("hdmg") {
                        half_dmg_range = x.as_str().parse::<i32>().unwrap().into()
                    }
                } else if let Some(x) = RX_R_MAX.captures(d) {
                    max_range = x.name("max").unwrap().as_str().parse::<i32>().unwrap().into()
                } else if let Some(x) = RX_R_SHOTS.captures(d) {
                    shots = shots_from_captures(x).into()
                } else if let Some(x) = RX_R_MIN.captures(d) {
                    min_range = x.name("min").unwrap().as_str().parse::<i32>().unwrap().into()
                } else if let Some(x) = RX_R_ST.captures(d) {
                    if let Some(x) = x.name("st") {
                        st_req = x.as_str().parse::<i32>().unwrap().into()
                    } else {
                        tripod = true
                    }
                } else if let Some(x) = RX_MAX_DMG.captures(d) {
                    max_damage = Some(DamageDelivery::Dice(
                        x.name("dmgd").unwrap().as_str().parse::<i32>().unwrap(),
                        if let Some(x) = x.name("dmgb") {
                            x.as_str().parse::<i32>().unwrap()
                        } else {0}
                    ))
                } else if let Some(x) = RX_19XX.captures(d) {
                    rl_year = x.get(0).unwrap().as_str().parse::<i32>().unwrap().into()
                } else if let Some(x) = RX_TL.captures(d) {
                    tl = x.name("tl").unwrap().as_str().parse::<i32>().unwrap().into()
                } else if let Some(x) = RX_LEGALITY.captures(d) {
                    lc = x.name("lc").unwrap().as_str().parse::<i32>().unwrap().into()
                } else if let Some(x) = RX_COUNTRY.captures(d) {
                    rl_country = x.get(0).unwrap().as_str().to_string().into()
                } else {
                    todo!("Unknown: {d}")
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

    Ranged { name: value.0.trim().to_string(), damage, max_damage,
            cost, weight, skill, notes, mod_groups, ss, acc, rof,
            rcl, min_range, half_dmg_range, max_range, shots, st_req,
            rl_year, rl_country, tripod, tl, lc,
    }
}


#[cfg(test)]
mod ranged_tests {
    use gurpschgen_lib::{damage::{Damage, DamageDelivery}, equipment::weapon::ranged::rof::RoF, misc::damaged::Damaged};

    use crate::ranged::ranged_wpn_from_tuple;

    #[test]
    fn ranged_1_works() {
        let data = ("  AT-3 Sagger (ATGM)  ", "Cr/48+0, Acc+14, SS 23, Min 300, 1/2D n/a, Max 3000, RoF 1/10, Shots 1; 20000,260");
        let rng = ranged_wpn_from_tuple(data);
        assert_eq!("AT-3 Sagger (ATGM)", rng.name);
        assert!(rng.damage.contains(&Damage::Cr(DamageDelivery::Dice(48, 0))));
        assert_eq!(&RoF::Slow(1, 10), rng.rof.as_ref().unwrap());
        assert_eq!(14, rng.acc);
    }

    #[test]
    fn ranged_2_works() {
        let data = ("  IMI Eagle .50AE", "Cr/3+2(X1.5), Acc+3, RoF 3~, ST 13, Rcl-4, Shots 9+1; 1000,4.5; Guns: Pistol");
        let rng = ranged_wpn_from_tuple(data);
        assert_eq!("IMI Eagle .50AE", rng.name);
        assert!(rng.damage.contains(&Damage::Cr(DamageDelivery::DiceMul(3, 2, 1.5))));
        assert_eq!(&RoF::SemiAuto(3), rng.rof.as_ref().unwrap());
        assert_eq!(&13, rng.st_req.as_ref().unwrap());
    }

    #[test]
    fn ranged_3_works() {
        let data = ("  EX34 Chain Gun 7.62x51mm  ", "Cr/7+0, Acc+15, SS 20, RoF 9, Shots 500Box, ST XX(Tripod), Rcl -1; 5000,32.0");
        let rng = ranged_wpn_from_tuple(data);
        assert_eq!("EX34 Chain Gun 7.62x51mm", rng.name);
        assert!(rng.damage.contains(&Damage::Cr(DamageDelivery::Dice(7, 0))));
        assert_eq!(15, rng.acc);
        assert_eq!(&20, rng.ss.as_ref().unwrap());
        assert_eq!(&RoF::FullAuto(9), rng.rof.as_ref().unwrap());
    }

    #[test]
    fn max_dmg_works() {
        let data = ("  EX34 Chain Gun 7.62x51mm  ", "Cr/7+0, Acc+15, max dmg 1+2, SS 20, RoF 9, Shots 500Box, ST XX(Tripod), Rcl -1; 5000,32.0");
        let wpn = ranged_wpn_from_tuple(data);
        assert_eq!(DamageDelivery::Dice(1, 2), wpn.max_damage().clone().unwrap());
    }
}
