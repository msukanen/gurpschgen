use std::collections::HashMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum DamageType {
    Cut,
    Cr,
    Imp,
}

#[derive(Debug, Clone)]
pub enum DamageResistance {
    All(i32),
    Variable(HashMap<DamageType, i32>),
}

impl DamageResistance {
    pub fn value(&self) -> i32 {
        match self {
            Self::All(v) => *v,
            _ => todo!("hashmap?!")
        }
    }
}

#[derive(Debug, Clone)]
pub enum PassiveDefense {
    All(i32),
    Variable(HashMap<DamageType, i32>),
}

impl PassiveDefense {
    pub fn value(&self) -> i32 {
        match self {
            Self::All(v) => *v,
            _ => todo!("hashmap?!")
        }
    }
}

/**
 General damage types + embedded delivery method.
 */
#[derive(Debug, Clone)]
pub enum Damage {
    Cut(DamageDelivery),
    Cr(DamageDelivery),
    Imp(DamageDelivery),
}

/**
 Some common damage delivery methods.
 */
#[derive(Debug, Clone)]
pub enum DamageDelivery {
    /**
     **Fixed**: num dice & modifier. Guns and other weapons that have relatively stable/fixed dmg model.
     */
    Fixed(i32, i32),
    /**
     **Sw**ing: based on ST. Embedded modifier.
     */
    Sw(i32),
    /**
     **Thr**ust: based on ST. Embedded modifier.
     */
    Thr(i32),
}
