use serde::{Deserialize, Serialize};

use crate::{attrib::AttributeType, misc::{costly::HasCost, named::HasName}};

/// Skill difficulty factor.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum DifficultyRating {
    /// Easy.
    E,
    /// Average.
    A,
    /// Hard.
    H,
    /// Special — usually associated with martial arts' maneuvers.
    S,
    /// Very Hard.
    VH,
}

/// Legacy Skill 'base'/'root'.
/// 
/// **Note**: for "from 3rd ed to 4th ed" converter use only.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum LegacySkillRoot {
    /// Mental.
    M { stat: AttributeType, diff: DifficultyRating },
    /// Martial Arts' maneuver (or some other sort of a "sub-skill").
    MA { diff: DifficultyRating },
    /// Physical.
    P { stat: AttributeType, diff: DifficultyRating },
}

impl From<LegacySkillRoot> for DifficultyRating {
    /// Extract [DifficultyRating] from [LegacySkillRoot].
    fn from(value: LegacySkillRoot) -> Self {
        match value {
            LegacySkillRoot::M { diff ,..}|
            LegacySkillRoot::P { diff,..}|
            LegacySkillRoot::MA { diff } => diff
        }
    }
}

/// Skill defaulting modes.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum SkillDefault {
    /// Multiplicative default.
    Mul { at: String, val: f64 },
    /// Divisive default.
    Div { at: String, val: f64 },
    /// Additive (or subtractive) default.
    Add { at: String, val: i32 },
}

/// A struct for e.g. skills and spells.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Skill {
    /// Name of the skill, obviously.
    pub name: String,
    /// No# of ranks in the skill.
    pub rank: usize,
    /// mental/physical, difficulty, etc.
    pub diff: DifficultyRating,
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

impl HasName for Skill {
    fn name(&self) -> &str {
        &self.name
    }
}

impl HasCost for Skill {
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
    /// Get skill level.
    fn level(&self) -> Option<i32>;
}

impl SkillLevel for Skill {
    fn level(&self) -> Option<i32> {
        match &self.diff {
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
                ..=0 => //TODO: see if skill has a/any default or not.
                        (-6).into(),
                n => ((n as i32) - 4).into(),
            },

            DifficultyRating::S => todo!("Ed4 x/S")
        }
    }
}
