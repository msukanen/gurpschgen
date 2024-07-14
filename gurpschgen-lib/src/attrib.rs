use std::{cmp::max, collections::HashMap, ops::{Add, Sub}};

use crate::{misc::costly::Costly, modifier::{Modifier, ModifierValue}};

#[derive(Hash, PartialEq, Eq)]
pub enum AttributeType {
    DX, HT, IQ, ST,
}

#[derive(Debug, Clone)]
pub enum Attribute {
    DX(AttributeValue),
    HT(AttributeValue),
    IQ(AttributeValue),
    ST(AttributeValue),
}

#[derive(Debug, Clone)]
pub struct AttributeValue {
    base_val: i32,
    rel_val: i32,
    pub modifiers: HashMap<Modifier, Option<ModifierValue>>,
}

pub trait AttributeValued {
    /**
     Get attribute's base/root value.
     */
    fn base_val(&self) -> i32;
    /**
     Get attribute's relative (+/-) value.
     */
    fn rel_val(&self) -> i32;
    /**
     Get attribute's effective value.

     To change effective value, use `set_base_val()` or (in most cases) `set_rel_val()` respectively (if such exist).
     */
    fn value(&self) -> i32 {
        self.base_val() + self.rel_val()
    }
}

impl AttributeValue {
    /**
     Set base value. Note that a `value` less than `1` will be treated as `1`.

     **Params**
     * `value` - new base value; ≤1 → 1
     */
    pub fn set_base_val(&mut self, value: i32) -> &Self {
        self.base_val = max(1, value);
        self
    }
    /**
     Set relative value. Relative value will be rejiggled if it'd bring effective value down to ≤0.

     **Params**
     * `value` - new relative value.
     */
    pub fn set_rel_val(&mut self, value: i32) -> &Self {
        self.rel_val = max(-(self.base_val() - 1), value);
        self
    }
}

impl AttributeValued for AttributeValue {
    fn base_val(&self) -> i32 { self.base_val }
    fn rel_val(&self) -> i32 { self.rel_val }
}

impl Add<i32> for AttributeValue {
    type Output = Self;
    /**
     Add `rhs` to `rel_value()` of `self`.
     */
    fn add(self, rhs: i32) -> Self::Output {
        Self {
            rel_val: max(-(self.base_val - 1), self.rel_val + rhs),
            base_val: self.base_val,
            modifiers: self.modifiers,
        }
    }
}

impl Sub<i32> for AttributeValue {
    type Output = Self;
    /**
     Subtract `rhs` from `rel_value()` of `self`.
     */
    fn sub(self, rhs: i32) -> Self::Output {
        self + (-rhs)
    }
}

impl Attribute {
    /**
     Instantiate a new [Attribute].

     **Params**
     * `attrib_type` - attribute's [type][AttributeType].
     * `base_val` - desired base value. Note: might get adjusted.
     * `rel_val` - desired relative value. Note: might get adjusted.
     * `modifiers` - any and all to-be-applied [Modifier].
     */
    pub fn new(
        attrib_type: AttributeType,
        base_val: i32,
        rel_val: i32,
        modifiers: Option<HashMap<Modifier, Option<ModifierValue>>>
    ) -> Self {
        // Base val cannot be less than 1.
        let base_val = max(1, base_val);
        // Relval isn't allowed to bring effective value ≤ 0.
        let rel_val = max(-(base_val - 1), rel_val);

        let attrib_value = AttributeValue {
            base_val,
            rel_val,
            modifiers: if let Some(m) = modifiers {m} else {HashMap::new()},
        };

        match attrib_type {
            AttributeType::DX => Attribute::DX(attrib_value),
            AttributeType::HT => Attribute::HT(attrib_value),
            AttributeType::IQ => Attribute::IQ(attrib_value),
            AttributeType::ST => Attribute::ST(attrib_value),
        }
    }

    /**
     Instantiate a new [Attribute] using default values.

     **Params**
     * `attrib_type` - attribute's [type][AttributeType].
     */
    pub fn default(attrib_type: AttributeType) -> Self {
        Self::new(attrib_type, 10, 0, None)
    }
}

impl AttributeValued for Attribute {
    fn base_val(&self) -> i32 {
        match self {
            Self::DX(v) |
            Self::HT(v) |
            Self::IQ(v) |
            Self::ST(v) => v.base_val()
        }
    }

    fn rel_val(&self) -> i32 {
        match self {
            Self::DX(v) |
            Self::HT(v) |
            Self::IQ(v) |
            Self::ST(v) => v.rel_val()
        }
    }
}

impl Costly for Attribute {
    fn cost(&self) -> f64 {
        let mut cost: f64;
        match self {
            Self::DX(v) => cost = 20.0 * v.rel_val() as f64,
            Self::HT(v) => cost = 10.0 * v.rel_val() as f64,
            Self::IQ(v) => cost = 20.0 * v.rel_val() as f64,
            Self::ST(v) => {
                cost = 10.0 * v.rel_val() as f64;
                if v.modifiers.contains_key(&Modifier::NoFineManipulators) {
                    cost *= 0.6
                }
                if let Some(m) = v.modifiers.get(&Modifier::Size) {
                    match m {
                        Some(ModifierValue::I(v)) => cost *= 1.0 + 0.1 * max(-8, *v) as f64,
                        _ => ()
                    }
                }
            },
        }
        cost
    }
}

impl Add<i32> for Attribute {
    type Output = Self;
    /**
     Add `rhs` to `self`, producing a new [Attribute] while at it.
     */
    fn add(self, rhs: i32) -> Self::Output {
        match self {
            Self::DX(v) => Self::DX(v + rhs),
            Self::HT(v) => Self::HT(v + rhs),
            Self::IQ(v) => Self::IQ(v + rhs),
            Self::ST(v) => Self::ST(v + rhs),
        }
    }
}

impl Sub<i32> for Attribute {
    type Output = Self;
    /**
     Subtract `rhs` from `self`, producing a new [Attribute] while at it.
     */
    fn sub(self, rhs: i32) -> Self::Output {
        match self {
            Self::DX(v) => Self::DX(v - rhs),
            Self::HT(v) => Self::HT(v - rhs),
            Self::IQ(v) => Self::IQ(v - rhs),
            Self::ST(v) => Self::ST(v - rhs),
        }
    }
}

#[cfg(test)]
mod attrib_tests {
    use crate::{attrib::AttributeValued, misc::costly::Costly};

    use super::{Attribute, AttributeType};

    #[test]
    fn defaults_work() {
        let a = Attribute::default(AttributeType::DX);
        assert_eq!(10, a.base_val());
        assert_eq!(0, a.rel_val());
        assert_eq!(10, a.value());
        assert_eq!(0.0, a.cost());
    }

    #[test]
    fn addition_works() {
        let a = Attribute::default(AttributeType::DX);
        let a = a + 2;
        assert_eq!(2, a.rel_val());
        assert_eq!(40.0, a.cost());
    }

    #[test]
    fn subtraction_works() {
        
    }
}
