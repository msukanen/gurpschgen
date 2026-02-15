use std::{cmp::max, collections::HashMap, ops::{Add, AddAssign, Sub, SubAssign}};

use serde::{Deserialize, Serialize};

use crate::{attrib::value::GURPS_STAT_BASE, misc::costly::HasCost, modifier::{Modifier, ModifierValue}};

pub mod r#type;
pub use r#type::AttributeType;
pub mod payload;
pub use payload::AttributePayload;
pub mod value;
pub use value::{AttributeValue, HasAttributeValue};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Attribute {
    DX(AttributeValue, AttributePayload),
    HT(AttributeValue, AttributePayload),
    IQ(AttributeValue, AttributePayload),
    ST(AttributeValue, AttributePayload),
}

impl Attribute {
    /// Instantiate a new [Attribute].
    /// 
    /// # Args
    /// - `attrib_type`: attribute's [type][AttributeType].
    /// - `base_val`: desired base value. Note: might get adjusted.
    /// - `rel_val`: desired relative value. Note: might get adjusted.
    /// - `modifiers`: any and all to-be-applied [Modifier], if any to begin with…
    pub fn new(
        attrib_type: AttributeType,
        base_val: i32,
        rel_val: i32,
        modifiers: Option<HashMap<Modifier, Option<ModifierValue>>>
    ) -> Self {
        let attrib_value = AttributeValue {
            // base_val cannot be less than 1 (for a PC at least).
            base_val: 1.max(base_val),
            // rel_val isn't allowed to bring effective value ≤0. Adjust accordingly if needed.
            rel_val: max(-(base_val - 1), rel_val),
        };

        let payload = AttributePayload {
            modifiers: modifiers.unwrap_or_else(|| HashMap::new()),
        };

        match attrib_type {
            AttributeType::DX => Attribute::DX(attrib_value, payload),
            AttributeType::HT => Attribute::HT(attrib_value, payload),
            AttributeType::IQ => Attribute::IQ(attrib_value, payload),
            AttributeType::ST => Attribute::ST(attrib_value, payload),
        }
    }

    /// Instantiate a new [Attribute] of [`attrib_type`][AttributeType] using default values.
    /// 
    /// # Args
    /// - `attrib_type`: attribute's [type][AttributeType].
    pub fn default(attrib_type: AttributeType) -> Self {
        Self::new(attrib_type, GURPS_STAT_BASE, 0, None)
    }

    /// Set a `modifier`.
    /// 
    /// # Args
    /// - `m,v`: a [Modifier]/[ModifierValue] pair, with value being optional.
    /// 
    /// # Returns
    /// `&mut self` for chaining purposes.
    pub fn set_modifier(&mut self, (m, v): (Modifier, Option<ModifierValue>)) -> &mut Self {
        match self {
            Self::DX(_, p) |
            Self::HT(_, p) |
            Self::IQ(_, p) |
            Self::ST(_, p) => p.modifiers.insert(m, v),
        };
        self
    }

    /// Unset a modifier. Nothing, of course, happens if said `modifier` is not present at all.
    /// 
    /// # Args
    /// - `modifier`: [Modifier] to unset.
    /// 
    /// # Returns
    /// `&mut self` for chaining purposes.
    pub fn unset_modifier(&mut self, modifier: Modifier) -> &mut Self {
        match self {
            Self::DX(_, p) |
            Self::HT(_, p) |
            Self::IQ(_, p) |
            Self::ST(_, p) => p.modifiers.remove(&modifier),
        };
        self
    }
}

impl HasAttributeValue for Attribute {
    fn base_val(&self) -> i32 {
        match self {
            Self::DX(v, _) |
            Self::HT(v, _) |
            Self::IQ(v, _) |
            Self::ST(v, _) => v.base_val()
        }
    }

    fn relative_value(&self) -> i32 {
        match self {
            Self::DX(v, _) |
            Self::HT(v, _) |
            Self::IQ(v, _) |
            Self::ST(v, _) => v.relative_value()
        }
    }
}

impl HasCost for Attribute {
    fn cost(&self) -> f64 {
        match self {
            Self::DX(v,_) |
            Self::IQ(v,_) => 20.0 * v.relative_value() as f64,
            
            Self::HT(v,_) => 10.0 * v.relative_value() as f64,
            
            Self::ST(value, payload) => {
                let base_cost = 10.0 * value.relative_value() as f64;
                let nfm_mult = if payload.modifiers.contains_key(&Modifier::NoFineManipulators) { 0.6 } else { 1.0 };
                let size_mult = payload.modifiers.get(&Modifier::Size)
                    .and_then(|m| if let Some(ModifierValue::I(v)) = m {Some(*v)} else {None})
                    .map(|v| 1.0 + 0.1 * v.max(-8) as f64)
                    .unwrap_or(1.0);
                base_cost * nfm_mult * size_mult
            },
        }
    }
}

impl Add<i32> for Attribute {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        match self {
            Self::DX(v, p) => Self::DX(v + rhs, p),
            Self::HT(v, p) => Self::HT(v + rhs, p),
            Self::IQ(v, p) => Self::IQ(v + rhs, p),
            Self::ST(v, p) => Self::ST(v + rhs, p),
        }
    }
}

impl Sub<i32> for Attribute {
    type Output = Self;
    fn sub(self, rhs: i32) -> Self::Output {
        match self {
            Self::DX(v, p) => Self::DX(v - rhs, p),
            Self::HT(v, p) => Self::HT(v - rhs, p),
            Self::IQ(v, p) => Self::IQ(v - rhs, p),
            Self::ST(v, p) => Self::ST(v - rhs, p),
        }
    }
}

impl AddAssign<i32> for Attribute {
    fn add_assign(&mut self, rhs: i32) {
        match self {
            Self::DX(v, _) |
            Self::HT(v, _) |
            Self::IQ(v, _) |
            Self::ST(v, _) => *v += rhs
        };
    }
}

impl SubAssign<i32> for Attribute {
    fn sub_assign(&mut self, rhs: i32) {
        match self {
            Self::DX(v, _) |
            Self::HT(v, _) |
            Self::IQ(v, _) |
            Self::ST(v, _) => *v -= rhs
        };
    }
}

impl PartialEq<i32> for Attribute {
    fn eq(&self, other: &i32) -> bool {
        self.value().eq(other)
    }
}

impl PartialEq<&Attribute> for i32 {
    fn eq(&self, other: &&Attribute) -> bool {
        other.value().eq(self)
    }
}

impl PartialEq<Attribute> for i32 {
    fn eq(&self, other: &Attribute) -> bool {
        other.value().eq(self)
    }
}

#[cfg(test)]
mod attrib_tests {
    use crate::{attrib::HasAttributeValue, misc::{approx::Approx, costly::HasCost}, modifier::{Modifier, ModifierValue}};

    use super::{Attribute, AttributeType};

    #[test]
    fn defaults_work() {
        let a = Attribute::default(AttributeType::DX);
        assert_eq!(10, a.base_val());
        assert_eq!(0, a.relative_value());
        assert_eq!(10, a.value());
        assert_eq!(0.0, a.cost());
    }

    #[test]
    fn addition_works() {
        let a = Attribute::default(AttributeType::DX);
        let a = a + 2;
        assert_eq!(2, a.relative_value());
        assert_eq!(40.0, a.cost());
    }

    #[test]
    fn subtraction_works() {
        let a = Attribute::default(AttributeType::DX);
        let a = a - 2;
        assert_eq!(-2, a.relative_value());
        assert_eq!(-40.0, a.cost());
    }

    #[test]
    fn rel_val_clamping_works() {
        let a = Attribute::default(AttributeType::DX);
        let a = a - 10;// attempt to bring rel val of DX down to 0 (zero).
        assert_eq!(-9, a.relative_value());
        assert_eq!(-180.0, a.cost());
    }

    #[test]
    fn preset_modifier_works() {
        let mut a = Attribute::default(AttributeType::ST);
        a.set_modifier((Modifier::NoFineManipulators, None));
        a += 2;
        assert_eq!(2, a.relative_value());
        assert_eq!(12.0, a.cost());
    }

    #[test]
    fn set_modifier_chaining_works() {
        let mut a = Attribute::default(AttributeType::ST);
        a   .set_modifier((Modifier::NoFineManipulators, None))
            .set_modifier((Modifier::Size, Some(ModifierValue::I(-2))));
        a += 2;

        // We can't use assert_eq!() because of float imprecision and thus we're .approx()'ing here.
        assert!(a.cost().approx(9.6));
    }

    #[test]
    fn attrib_is_modifierless() {

    }
}
