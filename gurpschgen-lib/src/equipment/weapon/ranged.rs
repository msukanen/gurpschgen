pub mod rof;
pub mod shots;

use rof::RoF;

use regex::Regex;
use shots::Shots;

use crate::{damage::Damage, misc::{costly::Costly, mod_grouped::ModGrouped, noted::Noted, skilled::Skilled, weighed::Weighed}, RX_COST_WEIGHT, RX_DMGD};

thread_local! {
//Glock 20 10mm; Cr/3-2(X1.5), Acc+3, RoF 3~, ST 10, Rcl-2, Shots 15+1; 750,2.0; Guns: Pistol
//LPO-50; Cr/3+0, Acc+7, Max rnd. 70, RoF 1, Shots (3)9, Rcl -3; 150,52.0
//Mk19 AGL 40x53mm; Cr/3+0, Imp/3+0, Acc+15, RoF 6, Shots xxxB, ST XX(Tripod), Rcl -1; 6000,76
//M60 7.62x51mm; Cr/7+0, Acc+10, SS 19, RoF 10, Shots 100B, ST 13, Rcl -1; 3000,23.0
//NSV 12.7x108mm; Cr/12+0, Acc+15, SS 20, RoF 12, Shots 50, ST XX(Tripod), Rcl -1; 6000,120.0
//H-90 Rifle; Imp/2d6+1(2), SS 12, Acc+12, 1/2 250, Max 600, RoF 8, Shots 200/D; 3000, 7.0;Beam Weapons: Lasers; ;
    static RX_R_SS: Regex = Regex::new(r"(?:SS\s*(?<ss>[-+]?\d+))").unwrap();
    pub(crate) static RX_R_ACC: Regex = Regex::new(r"(?:\s*[aA]cc\s*(?<acc>[-+]?\d+))").unwrap();
    static RX_R_ROF: Regex = Regex::new(r"(?:[rR][oO][fF]\s+(?<rof>(?<rof1>\d+)(?:[*]|~|\/(?<rof2>\d+))?))").unwrap();
    static RX_R_RCL: Regex = Regex::new(r"(?:[rR]cl\s*(?<rcl>[-+]?\d+))").unwrap();
    // RX_R_HDMG will ignore all non-numeric 1/2 entries:
    static RX_R_HDMG: Regex = Regex::new(r"(?:1\/2D?\s+(?<hdmg>\d+))").unwrap();
    static RX_R_MIN: Regex = Regex::new(r"(?:(?:min|Min|MIN)\s+(?<min>\d+))").unwrap();
    static RX_R_MAX: Regex = Regex::new(r"(?:(?:max|Max|MAX)\s+(?<max>\d+))").unwrap();
    pub(crate) static RX_R_SHOTS: Regex = Regex::new(r"(?:[sS]hots\s+(?:(?:(?<battch>\d+)\/(?<batt>(?:A){1,3}|B|C|D|E|F))|(?:[(](?<fthrow1>\d+)[)])(?<fthrow2>\d+)|(?<xxxbelt>xxxB)|(?:(?<bfed>\d+)B(?<boxfed>ox)?)|(?:(?<splus>\d+)(?<splusmod>[+]\d+)?)))").unwrap();
    static RX_R_ST: Regex = Regex::new(r"(?:ST\s*(?<st>\d+))").unwrap();
}

/**
 Ranged weapon data.
 */
#[derive(Debug, Clone)]
pub struct Ranged {
    name: String,
    damage: Vec<Damage>,
    acc: i32,
    ss: Option<i32>,
    rof: Option<RoF>,
    rcl: Option<i32>,
    min_range: Option<i32>,
    half_dmg_range: Option<i32>,
    max_range: Option<i32>,
    st_req: Option<i32>,
    cost: Option<f64>,
    weight: Option<f64>,
    skill: Option<String>,
    notes: Option<String>,
    shots: Option<Shots>,
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
        let mut ss = None;
        let mut acc = 0;
        let mut rof = None;
        let mut rcl = None;
        let mut half_dmg_range = None;
        let mut max_range = None;
        let mut min_range = None;
        let mut shots = None;
        let mut st_req = None;
        for (index, x) in value.1.split(";").enumerate() {
            match index {
                0 => for d in x.split(",") {
                    let d = d.trim();
                    if let Some(x) = RX_DMGD.with(|rx| rx.captures(d)) {// TODO: this unfortunately will get repeated in Damage::from(). Fix somehow?
                        damage.push(Damage::from(x.get(0).unwrap().as_str()))
                    } else if let Some(x) = RX_R_ACC.with(|rx| rx.captures(d)) {
                        acc = x.name("acc").unwrap().as_str().parse::<i32>().unwrap()
                    } else if let Some(x) = RX_R_SS.with(|rx| rx.captures(d)) {
                        ss = x.name("ss").unwrap().as_str().parse::<i32>().unwrap().into()
                    } else if let Some(x) = RX_R_ROF.with(|rx| rx.captures(d)) {
                        rof = RoF::from(x).into()
                    } else if let Some(x) = RX_R_RCL.with(|rx| rx.captures(d)) {
                        rcl = x.name("rcl").unwrap().as_str().parse::<i32>().unwrap().into()
                    } else if let Some(x) = RX_R_HDMG.with(|rx| rx.captures(d)) {
                        half_dmg_range = x.name("hdmg").unwrap().as_str().parse::<i32>().unwrap().into()
                    } else if let Some(x) = RX_R_MAX.with(|rx| rx.captures(d)) {
                        max_range = x.name("max").unwrap().as_str().parse::<i32>().unwrap().into()
                    } else if let Some(x) = RX_R_SHOTS.with(|rx| rx.captures(d)) {
                        shots = Shots::from(x).into()
                    } else if let Some(x) = RX_R_MIN.with(|rx| rx.captures(d)) {
                        min_range = x.name("min").unwrap().as_str().parse::<i32>().unwrap().into()
                    } else if let Some(x) = RX_R_ST.with(|rx| rx.captures(d)) {
                        st_req = x.name("st").unwrap().as_str().parse::<i32>().unwrap().into()
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

        Self { name: value.0.trim().to_string(), damage, cost, weight, skill, notes, mod_groups, ss, acc, rof, rcl, min_range, half_dmg_range, max_range, shots, st_req }
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

#[cfg(test)]
mod ranged_tests {
    use crate::{damage::{Damage, DamageDelivery}, equipment::weapon::ranged::rof::RoF};

    use super::Ranged;

    #[test]
    fn ranged_1_works() {
        let data = ("  AT-3 Sagger (ATGM)  ", "Cr/48+0, Acc+14, SS 23, Min 300, 1/2D n/a, Max 3000, RoF 1/10, Shots 1; 20000,260");
        let rng = Ranged::from(data);
        assert_eq!("AT-3 Sagger (ATGM)", rng.name);
        assert!(rng.damage.contains(&Damage::Cr(DamageDelivery::Dice(48, 0))));
        assert_eq!(&RoF::Slow(1, 10), rng.rof.as_ref().unwrap());
        assert_eq!(14, rng.acc);
    }

    #[test]
    fn ranged_2_works() {
        let data = ("  IMI Eagle .50AE", "Cr/3+2(X1.5), Acc+3, RoF 3~, ST 13, Rcl-4, Shots 9+1; 1000,4.5; Guns: Pistol");
        let rng = Ranged::from(data);
        assert_eq!("IMI Eagle .50AE", rng.name);
        assert!(rng.damage.contains(&Damage::Cr(DamageDelivery::DiceMul(3, 2, 1.5))));
        assert_eq!(&RoF::SemiAuto(3), rng.rof.as_ref().unwrap());
        assert_eq!(&13, rng.st_req.as_ref().unwrap());
    }

    #[test]
    fn ranged_3_works() {
        let data = ("  EX34 Chain Gun 7.62x51mm  ", "Cr/7+0, Acc+15, SS 20, RoF 9, Shots 500Box, ST XX(Tripod), Rcl -1; 5000,32.0");
        let rng = Ranged::from(data);
        assert_eq!("EX34 Chain Gun 7.62x51mm", rng.name);
        assert!(rng.damage.contains(&Damage::Cr(DamageDelivery::Dice(7, 0))));
        assert_eq!(15, rng.acc);
        assert_eq!(&20, rng.ss.as_ref().unwrap());
        assert_eq!(&RoF::FullAuto(9), rng.rof.as_ref().unwrap());
    }
}
