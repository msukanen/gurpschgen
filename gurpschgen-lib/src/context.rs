use serde::{Deserialize, Serialize};

use crate::{adq::Adq, equipment::Equipment, skill::Skill};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum Context {
    Advantage,
    Bonus,
    Counter,
    Disadvantage,
    Equipment,
    Modifier,
    Package,
    Quirk,
    Skill,
    /// Spells are essentially [Context::Skill], but...
    Spell,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CategoryPayload {
    Advantage(Adq),
    Bonus(String),
    Counter(String),
    Disadvantage(Adq),
    Equipment(Equipment),
    Modifier(String),
    Package(Adq),
    Quirk(String),
    Skill(Skill),
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Advantage => "advantage",
            Self::Bonus => "bonus",
            Self::Counter => "counter",
            Self::Disadvantage => "disadvantage",
            Self::Equipment => "equipment",
            Self::Modifier => "modifier",
            Self::Package => "package",
            Self::Quirk => "quirk",
            Self::Skill => "skill",
            Self::Spell => "spell",
        })
    }
}
