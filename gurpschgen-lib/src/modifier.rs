#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Modifier {
    Size,
    NoFineManipulators,
}

#[derive(Debug, Clone)]
pub enum ModifierValue {
    I(i32),
    F(f64),
    Flat(Box<ModifierValue>),
    Ignore
}

impl ModifierValue {
    pub fn get(&self) -> f64 {
        match self {
            Self::F(f) => *f,
            Self::Flat(f) => f.get(),
            Self::I(i) => (*i) as f64,
            Self::Ignore => 1.0,
        }
    }
}
