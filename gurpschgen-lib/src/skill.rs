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
            Some(m) => match m.as_str() {
                "DX" => Self::DX,
                "HT" => Self::HT,
                "IQ" => Self::IQ,
                "ST" => Self::ST,
                n => todo!("FATAL: base stat \"{n}\" not recognized!")
            }
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
        static RX_SKILL_BASE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?:\s*(?<base>MA?|P)\/(?<diff>E|A|V?H(?:\s*\((?<stat>DX|HT|IQ|ST)\))?))").unwrap());
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
 A struct for both Skills &amp; Spells.
 */
#[derive(Debug, Clone, PartialEq)]
pub struct Skill {
    name: String,
    rank: usize,
    base: Base,
    defaults: Vec<(String, i32)>,
    affected_by_bonuses: Vec<String>,
    tl_dependant: bool,
    increases_counters: Vec<String>,
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
        static RX_TL: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:TL)").unwrap());
        static RX_GBONUS: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:(?:(?<bv1>[-+]\d+)\s+(?<bname1>\w+.*))|(?:(?<bname2>.+)(?<bv2>[-+]\d+)\s*$))").unwrap());
        
        let name = value.0.trim();
        let mut base = None;
        let mut defaults = vec![];
        let mut bonus = vec![];
        let mut tl_dependant = false;
        let mut counters = vec![];
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
                            defaults.push((
                                x.name("name").unwrap().as_str().trim().to_string(),
                                x.name("def").unwrap().as_str().parse::<i32>().unwrap()
                            ))
                        } else {
                            todo!("Unrecognized: \"{d}\"")
                        }
                    }
                },
                2 => {
                    println!("{}: 2→ {x}", value.0)
                },
                // Bonuses affecting the skill.
                3 => {
                    let bs = x.split(",");
                    for b in bs {
                        let b = b.trim();
                        if !b.is_empty() {
                            bonus.push(b.to_string())
                        }
                    }
                },
                // Counters the skill counts as...
                4 => {
                    let cs = x.split(",");
                    for c in cs {
                        let c = c.trim();
                        if !c.is_empty() {
                            counters.push(c.to_string())
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
                                gives.push((b.as_str().trim().to_string(), v.as_str().parse::<i32>().unwrap()))
                            } else /*bname2*/{
                                let Some(b) = x.name("bname2") else {panic!("FATAL: regex fail?!")};
                                let Some(v) = x.name("bv2") else {panic!("FATAL: regex fail?!")};
                                gives.push((b.as_str().trim().to_string(), v.as_str().parse::<i32>().unwrap()))
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
            defaults, affected_by_bonuses: bonus,
            tl_dependant, increases_counters: counters,
            gives_bonuses: gives,
        }
    }
}

#[cfg(test)]
mod skill_tests {
    use super::Skill;

    #[test]
    #[should_panic]
    fn very_basics_panic() {
        let data = ("Cadmus", "MA/H; Alchemy+0, Digity-2, Dignus 4, Dignus B +3");
        let sk = Skill::from(data);
    }

    #[test]
    fn very_basics() {
        let data = ("<test>", "MA/H; Alchemy+0, Digity-2, Dignus B +3");
        let sk = Skill::from(data);
    }

    #[test]
    fn more_complex() {
        let data = ("Karate", "P/H; Karate Art-3, Karate Sport-3; ; +Melee Weapon Bonus; ; +5 Punching Damage Bonus, Kicking Damage Bonus +5");
        let sk = Skill::from(data);
        assert_eq!(vec![("Karate Art".to_string(), -3), ("Karate Sport".to_string(), -3)], sk.defaults);
        assert_eq!(vec![("Punching Damage Bonus".to_string(), 5), ("Kicking Damage Bonus".to_string(), 5)], sk.gives_bonuses);
    }
}
