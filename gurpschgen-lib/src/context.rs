use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::misc::category::Category;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum Context {
    Advantage,
    Bonus,
    Counter,
    Disadvantage,
    Equipment,
    Genre,
    Modifier,
    Package,
    Quirk,
    Skill,
    /// Spells are essentially [Context::Skill], but...
    Spell,
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Advantage => "advantage",
            Self::Bonus => "bonus",
            Self::Counter => "counter",
            Self::Disadvantage => "disadvantage",
            Self::Equipment => "equipment",
            Self::Genre => "genre",
            Self::Modifier => "modifier",
            Self::Package => "package",
            Self::Quirk => "quirk",
            Self::Skill => "skill",
            Self::Spell => "spell",
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ContextPayload {
    pub context: Context,
    pub items: HashMap<String, Category>,
}

impl ContextPayload {
    pub fn new(context: Context) -> Self {
        ContextPayload { context, items: HashMap::new() }
    }
}
