use serde::{Deserialize, Serialize};

use crate::{config::Config, edition::GurpsEd, misc::{costly::Costly, named::Named}};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Stat {
    DX, HT, IQ, ST
}

/**
 Skill difficulty factor.
 */
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum DifficultyRating {
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

/**
 Skill 'base'/'root'.
 */
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum SkillRoot {
    /// Mental.
    M { stat: Stat, diff: DifficultyRating },
    /// Martial Arts' maneuver (or some other sort of a "sub-skill").
    MA { diff: DifficultyRating },
    /// Physical.
    P { stat: Stat, diff: DifficultyRating },
}

/**
 Skill defaulting modes.
 */
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum SkillDefault {
    /// Multiplicative default.
    Mul { at: String, val: f64 },
    /// Divisive default.
    Div { at: String, val: f64 },
    /// Additive (or subtractive) default.
    Add { at: String, val: i32 },
}

/**
 A struct for both Skills &amp; Spells.
 */
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Skill {
    /// Name of the skill, obviously.
    pub name: String,
    /// No# of ranks in the skill.
    pub rank: usize,
    /// mental/physical, difficulty, etc.
    pub base: SkillRoot,
    /// What the skill defaults to...
    pub defaults: Vec<SkillDefault>,
    /// The bonuses the final skill level is affected by...
    pub affected_by_bonuses: Vec<String>,
    /// There's TL-dependant variant(s) of the skill?
    pub tl_dependant: bool,
    /// Counter(s) which choosing the skill increases, if any.
    pub increases_counters: Vec<String>,
    /// Other skills, etc. Used mainly for e.g. "profession"-packages.
    pub gives: Vec<(String, i32)>,
    /// Dmg bonus, etc., what the skill levels give.
    pub gives_bonuses: Vec<(String, i32)>,
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
                SkillRoot::M { diff: d, ..} => match d {
                    DifficultyRating::E => (match self.rank {
                        ..=0 => -3,
                        n => (n as i32) - 1
                    }).into(),

                    DifficultyRating::A => (match self.rank {
                        ..=0 => -4,
                        n => (n as i32) - 2
                    }).into(),

                    DifficultyRating::H => (match self.rank {
                        ..=0 => -5,
                        n => (n as i32) - 3
                    }).into(),

                    DifficultyRating::VH => match self.rank {
                        ..=0 => //TODO: check if skill has default or not.
                                (-6).into(),
                        n => ((n as i32) - 4).into()
                    },

                    DifficultyRating::S => todo!("Ed3 x/S")
                },

                SkillRoot::MA { diff: d} => todo!("Base::MA(d)"),
                SkillRoot::P { diff: d, ..} => todo!("Base::P(_,d)")
            },

            GurpsEd::Ed4 => match &self.base {
                SkillRoot::M { diff: d, ..} |
                SkillRoot::MA { diff: d, ..} |
                SkillRoot::P { diff: d, ..} => match d {
                    DifficultyRating::E => (match self.rank {
                        ..=0 => -4,
                        n => (n as i32) - 1,
                    }).into(),
                    
                    DifficultyRating::A => (match self.rank {
                        ..=0 => -5,
                        n => (n as i32) - 2,
                    }).into(),

                    DifficultyRating::H => (match self.rank {
                        ..=0 => -6,
                        n => (n as i32) - 3,
                    }).into(),

                    DifficultyRating::VH => match self.rank {
                        ..=0 => //TODO: see if skill has default or not
                                (-6).into(),
                        n => ((n as i32) - 4).into(),
                    },

                    DifficultyRating::S => todo!("Ed4 x/S")
                }
            }
        }
    }
}
