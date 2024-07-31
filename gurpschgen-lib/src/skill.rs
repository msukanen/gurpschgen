use once_cell::sync::Lazy;
use regex::{Match, Regex};

use crate::{config::Config, edition::GurpsEd, misc::{costly::Costly, named::Named}};

#[derive(Debug, Clone, PartialEq)]
pub enum Stat {
    DX, HT, IQ, ST
}

impl From<(RootBase, Option<Match<'_>>)> for Stat {
    fn from(value: (RootBase, Option<Match<'_>>)) -> Self {
        match value.1 {
            None => match value.0 {
                RootBase::P => Self::DX,
                RootBase::M => Self::IQ
            },
            Some(m) => Self::from(m.as_str())
        }
    }
}

impl From<&str> for Stat {
    fn from(value: &str) -> Self {
        match value {
            "DX" => Self::DX,
            "HT" => Self::HT,
            "IQ" => Self::IQ,
            "ST" => Self::ST,
            n => todo!("FATAL: base stat \"{n}\" not recognized!")
        }
    }
}

/**
 Skill difficulty factor.
 */
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Difficulty {
    /// Easy.
    E,
    /// Average.
    A,
    /// Hard.
    H,
    /// Special &ndash; usually associated with martial arts' maneuvers.
    S,
    /// Very Hard.
    VH,
}

impl From<Option<Match<'_>>> for Difficulty {
    fn from(value: Option<Match<'_>>) -> Self {
        match value {
            None => panic!("FATAL: ?!"),
            Some(m) => match m.as_str() {
                "E" => Self::E,
                "A" => Self::A,
                "H" => Self::H,
                "S" => Self::S,
                "VH" => Self::VH,
                n => panic!("FATAL: unknown skill difficulty \"{n}\"!")
            }
        }
    }
}

/// Root base.
enum RootBase {
    /// Mental.
    M,
    /// Physical.
    P
}

/**
 Skill 'base'/'root'.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Base {
    /// Mental.
    M(Stat, Difficulty),
    /// Martial Arts' maneuver (or some other sort of a "sub-skill").
    MA(Difficulty),
    /// Physical.
    P(Stat, Difficulty,)
}

impl From<&str> for Base {
    fn from(value: &str) -> Self {
        static RX_SKILL_BASE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?:\s*(?<base>MA?|P)\/(?<diff>E|A|V?H|S)(?:\s*\((?<stat>DX|HT|IQ|ST)\))?)").unwrap());
        if let Some(caps) = RX_SKILL_BASE.captures(value) {
            let base = caps.name("base").unwrap().as_str();
            let stat = caps.name("stat");
            match base {
                "M" => Base::M(Stat::from((RootBase::M, stat)), Difficulty::from(caps.name("diff"))),
                "MA" => Base::MA(Difficulty::from(caps.name("diff"))),
                "P" => Base::P(Stat::from((RootBase::P, stat)), Difficulty::from(caps.name("diff"))),
                n => todo!("FATAL: base \"{n}\" not recognized!")
            }
        } else {
            todo!("FATAL: skill base \"{value}\" does not match specs (MA?|P)/(E|A|V?H)!")
        }
    }
}

/**
 Skill defaulting modes.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum SkillDefault {
    /// Multiplicative default.
    Mul(String, f64),
    /// Divisive default.
    Div(String, f64),
    /// Additive (or subtractive) default.
    Add(String, i32),
}

/**
 A struct for both Skills &amp; Spells.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Skill {
    /// Name of the skill, obviously.
    name: String,
    /// No# of ranks in the skill.
    rank: usize,
    /// mental/physical, difficulty, etc.
    base: Base,
    /// What the skill defaults to...
    defaults: Vec<SkillDefault>,
    /// The bonuses the final skill level is affected by...
    affected_by_bonuses: Vec<String>,
    /// There's TL-dependant variant(s) of the skill?
    tl_dependant: bool,
    /// Counter(s) which choosing the skill increases, if any.
    increases_counters: Vec<String>,
    /// Other skills, etc. Used mainly for e.g. "profession"-packages.
    gives: Vec<(String, i32)>,
    /// Dmg bonus, etc., what the skill levels give.
    gives_bonuses: Vec<(String, i32)>,
}

impl Named for Skill {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Costly for Skill {
    fn cost(&self) -> f64 {
        match self.rank {
            ..=0 => 0.0,
            1 => 1.0,
            2 => 2.0,
            n => (4 * n-2) as f64
        }
    }
}

pub trait SkillLevel {
    fn level(&self, config: &Config) -> Option<i32>;
}

impl SkillLevel for Skill {
    fn level(&self, config: &Config) -> Option<i32> {
        match config.edition {
            GurpsEd::Ed3 => match &self.base {
                Base::M(_, d) => match d {
                    Difficulty::E => (match self.rank {
                        ..=0 => -3,
                        n => (n as i32) - 1
                    }).into(),

                    Difficulty::A => (match self.rank {
                        ..=0 => -4,
                        n => (n as i32) - 2
                    }).into(),

                    Difficulty::H => (match self.rank {
                        ..=0 => -5,
                        n => (n as i32) - 3
                    }).into(),

                    Difficulty::VH => match self.rank {
                        ..=0 => //TODO: check if skill has default or not.
                                (-6).into(),
                        n => ((n as i32) - 4).into()
                    },

                    Difficulty::S => todo!("Ed3 x/S")
                },

                Base::MA(d) => todo!("Base::MA(d)"),
                Base::P(_, d) => todo!("Base::P(_,d)")
            },

            GurpsEd::Ed4 => match &self.base {
                Base::M(_, d) |
                Base::MA(d) |
                Base::P(_, d) => match d {
                    Difficulty::E => (match self.rank {
                        ..=0 => -4,
                        n => (n as i32) - 1,
                    }).into(),
                    
                    Difficulty::A => (match self.rank {
                        ..=0 => -5,
                        n => (n as i32) - 2,
                    }).into(),

                    Difficulty::H => (match self.rank {
                        ..=0 => -6,
                        n => (n as i32) - 3,
                    }).into(),

                    Difficulty::VH => match self.rank {
                        ..=0 => //TODO: see if skill has default or not
                                (-6).into(),
                        n => ((n as i32) - 4).into(),
                    },

                    Difficulty::S => todo!("Ed4 x/S")
                }
            }
        }
    }
}

impl From<(&str, &str)> for Skill {
    fn from(value: (&str, &str)) -> Self {
        static RX_DEF: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:\s*(?<name>.+)(?<def>[-+]\d+)\s*$)").unwrap());
        static RX_MAS_DEF: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:(?<what>.+)(?<mode>[-+*/])(?<val>\d+[.]?\d+)\s*$)").unwrap());
        static RX_TL: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:TL)").unwrap());
        static RX_GBONUS: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:(?:(?<bv1>[-+]\d+)\s+(?<bname1>\w+.*))|(?:(?<bname2>.+)(?<bv2>[-+]\d+)\s*$))").unwrap());
        static RX_GIVES: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:\s*(?<what>[^@]+)@(?<val>\d+))").unwrap());
        
        let name = value.0.trim();
        let mut base = None;
        let mut defaults = vec![];
        let mut affected_by_bonuses = vec![];
        let mut tl_dependant = false;
        let mut increases_counters = vec![];
        let mut gives_bonuses = vec![];
        let mut gives = vec![];

        for (index, x) in value.1.split(";").into_iter().enumerate() {
            let x = x.trim();
            if x.is_empty() { continue; }
            
            match index {
                // Skill base.
                0 => {
                    base = Base::from(x.trim()).into();
                    if let Some(_) = RX_TL.captures(x) {
                        tl_dependant = true
                    }
                },
                // Sort out skill default(s).
                1 => {
                    let ds = x.split(",");
                    for d in ds {
                        if let Some(x) = RX_DEF.captures(d) {
                            let v = if let Some(def) = x.name("def") {
                                def.as_str().parse::<i32>().unwrap()
                            } else {0};
                            defaults.push(SkillDefault::Add(x.name("name").unwrap().as_str().trim().to_string(), v))
                        } else if let Some(x) = RX_MAS_DEF.captures(d) {
                            let n = x.name("what").unwrap().as_str().trim();
                            let v = x.name("val").unwrap().as_str();
                            defaults.push(match x.name("mode").unwrap().as_str() {
                                "/" => SkillDefault::Div(n.to_string(), v.parse::<f64>().unwrap()),
                                _   => SkillDefault::Mul(n.to_string(), v.parse::<f64>().unwrap())
                            })
                        } else {
                            defaults.push(SkillDefault::Add(d.trim().to_string(), 0))
                        }
                    }
                },
                2 => {
                    let gs = x.split(",");
                    for g in gs {
                        if let Some(x) = RX_GIVES.captures(g) {
                            gives.push((
                                x.name("what").unwrap().as_str().trim().to_string(),
                                x.name("val").unwrap().as_str().parse::<i32>().unwrap()
                            ))
                        }
                    }
                },
                // Bonuses affecting the skill.
                3 => {
                    let bs = x.split(",");
                    for b in bs {
                        let b = b.trim();
                        if !b.is_empty() {
                            affected_by_bonuses.push(b.to_string())
                        }
                    }
                },
                // Counters the skill counts as...
                4|6 => {//TODO: is 6 actually for real, or is 
                        //    Storm (G) → M/H; ; Unus. Background:Grimoire Spells@1, Rain@12, Hail@12; ; Weather Water-Spells Count; ; Weather Air-Spells Count
                        // a bugged entry in original DTA?
                    let cs = x.split(",");
                    for c in cs {
                        let c = c.trim();
                        if !c.is_empty() {
                            increases_counters.push(c.to_string())
                        }
                    }
                },
                // Bonuses what the skill gives...
                5 => {
                    let bs = x.split(",");
                    for b in bs {
                        if let Some(x) = RX_GBONUS.captures(b) {
                            if let Some(b) = x.name("bname1") {
                                let Some(v) = x.name("bv1") else {panic!("FATAL: regex fail?!")};
                                gives_bonuses.push((b.as_str().trim().to_string(), v.as_str().parse::<i32>().unwrap()))
                            } else /*bname2*/{
                                let Some(b) = x.name("bname2") else {panic!("FATAL: regex fail?!")};
                                let Some(v) = x.name("bv2") else {panic!("FATAL: regex fail?!")};
                                gives_bonuses.push((b.as_str().trim().to_string(), v.as_str().parse::<i32>().unwrap()))
                            }
                        } else {
                            todo!("FATAL: unrecognized 5th for {}: \"{b}\"", value.0)
                        }
                    }
                }
                _=> todo!("{x}")
            }
        }

        Skill { rank: 0,
            name: name.to_string(),
            base: base.unwrap(),
            defaults, affected_by_bonuses,
            tl_dependant, increases_counters,
            gives_bonuses, gives,
        }
    }
}

#[cfg(test)]
mod skill_tests {
    use crate::skill::{Base, Difficulty, SkillDefault, Stat};

    use super::Skill;

    #[test]
    #[should_panic]
    fn very_basics_panic() {
        let data = ("Cadmus", "MA/H; Alchemy+0, Digity-2, Dignus 4, Dignus B +3");
        let sk = Skill::from(data);
    }

    #[test]
    fn very_basics() {
        let data = ("<test>", "M/H(ST); Alchemy+0, Digity-2, Dignus B +3");
        let sk = Skill::from(data);
        assert_eq!(Base::M(Stat::ST, Difficulty::H), sk.base);
    }

    #[test]
    fn more_complex() {
        let data = ("Karate", "P/H; Karate Art-3, Karate Sport-3; ; +Melee Weapon Bonus; ; +5 Punching Damage Bonus, Kicking Damage Bonus +5");
        let sk = Skill::from(data);
        assert_eq!(vec![
            SkillDefault::Add("Karate Art".to_string(), -3),
            SkillDefault::Add("Karate Sport".to_string(), -3)], sk.defaults);
        assert_eq!(vec![("Punching Damage Bonus".to_string(), 5), ("Kicking Damage Bonus".to_string(), 5)], sk.gives_bonuses);
    }

    #[test]
    fn lengthy_line_gives_works() {
        let data = ("INT", "M/E; IQ-0; Acting@13, Acrobatics@11, Administration@13, Blowpipe@12, Carousing@13, Computer Operation@14, Computer Programming@11, Criminology@11, Cryptanalysis@10, Dancing@10, Demolition@11, Detect Lies@13, Diagnosis@9, Diplomacy@13, Disguise@11, Electronics Operation: Communications@12, Electronics Operation: Security Systems@14, Escape@10, Explosive Ordnance Disposal@10, Fast Draw: Pistol@13, Fast Draw: Knife@11, Fast-Talk@13, First Aid@12, Forensics@10, Forgery@12, Gesture@13, Guns: Pistol@15, Guns: Submachine Gun@11, Holdout@13, Intelligence Analysis@13, Interrogation@12, Judo@11, Knife@10, Lockpicking@10, Motorcycle@10, Photography@11, Pickpocket@10, Poisons@9, Research@13, Sex Appeal@12, Shadowing@13, Shortsword@9, SIGINT Collection and Jamming@10, Sign Language@12, Skiing@9, Stealth@12, Streetwise@12, Swimming@13, Throwing@9, Tracking@10, Traffic Analysis@12, Traps@11;");
        let sk = Skill::from(data);
        let mut found = false;
        for x in sk.gives {
            if x.0.eq("Computer Programming") && x.1.eq(&11) {
                found = true;
                break;
            }
        }
        assert_eq!(true, found);
    }

    #[test]
    fn defaults_work_without_explicit_value_given() {
        let data = ("Beam Weapons: Lasers", "P/E, TL; DX-4, Beam Weapons: Electrolasers-4, Beam Weapons: Blasters-4, Beam Weapons: Flamers-4, Beam Weapons: Sonic-4, Beam Weapons: Neural-4, Beam Weapons: Force Beams; ; +High IQ Guns Bonus");
        let sk = Skill::from(data);
        assert_eq!(vec![
            SkillDefault::Add("DX".to_string(), -4),
            SkillDefault::Add("Beam Weapons: Electrolasers".to_string(),-4),
            SkillDefault::Add("Beam Weapons: Blasters".to_string(),-4),
            SkillDefault::Add("Beam Weapons: Flamers".to_string(),-4),
            SkillDefault::Add("Beam Weapons: Sonic".to_string(),-4),
            SkillDefault::Add("Beam Weapons: Neural".to_string(),-4),
            SkillDefault::Add("Beam Weapons: Force Beams".to_string(),0)], sk.defaults);
    }
}
