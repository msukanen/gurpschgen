use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum Modifier {
    Size,
    NoFineManipulators,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ModifierValue {
    I(i32),
    F(f64),
    Flat(Box<ModifierValue>),
    /// For [Modifier] which does not affect any value(s) directly by itself.
    Ignore
}

impl ModifierValue {
    pub fn get(&self) -> Option<f64> {
        match self {
            Self::F(f) => Some(*f),
            Self::Flat(f) => f.get(),
            Self::I(i) => Some((*i) as f64),
            Self::Ignore => None,
        }
    }
}
