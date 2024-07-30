use once_cell::sync::Lazy;
use regex::Regex;

use crate::{config::Config, edition::GurpsEd, misc::{costly::Costly, named::Named}};

#[derive(Debug, Clone)]
pub enum Stat {
    DX, HT, IQ, ST
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Difficulty {
    E, A, H, VH,
}

/**
 Skill 'base'/'root'.
 */
#[derive(Debug, Clone)]
pub enum Base {
    M(Stat, Difficulty),
    MA(Difficulty),
    P(Stat, Difficulty,)
}

impl From<&str> for Base {
    fn from(value: &str) -> Self {
        static RX_SKILL_BASE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?:\s*(?<base>MA?|P)\/(?<diff>E|A|V?H))").unwrap());
        if let Some(caps) = RX_SKILL_BASE.captures(value) {

        } else {
            panic!("FATAL: skill base \"{value}\" does not match specs (MA?|P)/(E|A|V?H)!")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Skill {
    name: String,
    rank: usize,
    base: Base,
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
                    }
                }
            }
        }
    }
}

impl From<(&str, &str)> for Skill {
    fn from(value: (&str, &str)) -> Self {
        let name = value.0.trim();
        let mut base = None;
        for (index, x) in value.1.split(";").into_iter().enumerate() {
            match index {
                0 => {
                    base = Base::from(x.trim()).into()
                },
                _=> todo!("")
            }
        }

        Skill {
            rank: 0,
            name: name.to_string(),
            base: base.unwrap(),
        }
    }
}
