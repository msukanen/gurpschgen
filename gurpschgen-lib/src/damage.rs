use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/**
 Damage types.
 */
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub enum DamageType {
    /// **Cut** &ndash; sharp blades, monowire, etc.
    Cut,
    /// **Cr**ush &ndash; blunt trauma, etc.
    Cr,
    Energy,// anything what "cauterizes" the wound instantly.
    /// **Imp**ale &ndash; puncture, arrows, spears, etc.
    Imp,
    /// **Var**iable damage, see your games' rules for details.
    Var,
    /// **Spec**ial damage, see your games' rules for details.
    Spec,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

impl From<i32> for DamageResistance {
    fn from(value: i32) -> Self {
        Self::All(value)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

impl From<i32> for PassiveDefense {
    fn from(value: i32) -> Self {
        Self::All(value)
    }
}

/**
 General damage types + embedded delivery method.
 */
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Damage {
    Cut(DamageDelivery),
    Cr(DamageDelivery),
    Energy(DamageDelivery),
    Imp(DamageDelivery),
    /// **Var**iable damage, see your games' rules for details.
    Var(DamageDelivery),
    /// **Spec**iable damage, see your games' rules for details.
    Spec(DamageDelivery),
}

/**
 Some common damage delivery methods.
 */
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum DamageDelivery {
    /// **Dice** & modifier. E.g. guns and other weapons that have relatively stable/fixed dmg model.
    Dice(i32, i32),
    /// About same as [Dice(x,y)][DamageDelivery::Dice] but with a multiplier for final delivered dmg.
    DiceMul(i32, i32, f64),
    /// **Flat**, binary dmg without any variation whatsoever &ndash; either does all or nothing at all.
    /// A very, very rare delivery.
    Flat(i32),
    /// **Sw**ing based on ST; embedded modifier.
    Sw(i32),
    /// *Thr**ust based on ST; embedded modifier.
    Thr(i32),
    /// **Var**iable damage, see your games' rules for details.
    Var,
    /// **Spec**ial damage, see your games' rules for details.
    Spec(i32),
}
