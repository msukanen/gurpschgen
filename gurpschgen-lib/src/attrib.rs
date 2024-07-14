use std::{cmp::max, collections::HashMap, ops::{Add, AddAssign, Sub, SubAssign}};

use crate::{misc::costly::Costly, modifier::{Modifier, ModifierValue}};

#[derive(Hash, PartialEq, Eq)]
pub enum AttributeType {
    DX, HT, IQ, ST,
}

#[derive(Debug, Clone)]
pub struct AttributePayload {
    modifiers: HashMap<Modifier, Option<ModifierValue>>,
}

#[derive(Debug, Clone)]
pub struct AttributeValue {
    base_val: i32,
    rel_val: i32,
}

#[derive(Debug, Clone)]
pub enum Attribute {
    DX(AttributeValue, AttributePayload),
    HT(AttributeValue, AttributePayload),
    IQ(AttributeValue, AttributePayload),
    ST(AttributeValue, AttributePayload),
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

impl AddAssign<i32> for AttributeValue {
    fn add_assign(&mut self, rhs: i32) {
        self.set_rel_val(self.rel_val + rhs);
    }
}

impl SubAssign<i32> for AttributeValue {
    fn sub_assign(&mut self, rhs: i32) {
        self.set_rel_val(self.rel_val - rhs);
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
        };

        let payload = AttributePayload {
            modifiers: if let Some(m) = modifiers {m} else {HashMap::new()},
        };

        match attrib_type {
            AttributeType::DX => Attribute::DX(attrib_value, payload),
            AttributeType::HT => Attribute::HT(attrib_value, payload),
            AttributeType::IQ => Attribute::IQ(attrib_value, payload),
            AttributeType::ST => Attribute::ST(attrib_value, payload),
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

    /**
     Set a `modifier`.

     **Params**
     * `modifier` - some sort of a [Modifier]/[ModifierValue] pair, for which value part is optional.
      
     **Returns** `&self` for chaining purposes.
     */
    pub fn set_modifier(&mut self, modifier: (Modifier, Option<ModifierValue>)) -> &mut Self {
        match self {
            Self::DX(_, p) |
            Self::HT(_, p) |
            Self::IQ(_, p) |
            Self::ST(_, p) => p.modifiers.insert(modifier.0, modifier.1),
        };
        self
    }

    /**
     Unset a modifier. Nothing, of course, happens if said `modifier` is not present at all.

     **Params**
     * `modifier` - [Modifier] to unset.
     
     **Returns** `&self` for chaining purposes.
     */
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

impl AttributeValued for Attribute {
    fn base_val(&self) -> i32 {
        match self {
            Self::DX(v, _) |
            Self::HT(v, _) |
            Self::IQ(v, _) |
            Self::ST(v, _) => v.base_val()
        }
    }

    fn rel_val(&self) -> i32 {
        match self {
            Self::DX(v, _) |
            Self::HT(v, _) |
            Self::IQ(v, _) |
            Self::ST(v, _) => v.rel_val()
        }
    }
}

impl Costly for Attribute {
    fn cost(&self) -> f64 {
        let mut cost: f64;
        match self {
            Self::DX(v, _) => cost = 20.0 * v.rel_val() as f64,
            Self::HT(v, _) => cost = 10.0 * v.rel_val() as f64,
            Self::IQ(v, _) => cost = 20.0 * v.rel_val() as f64,
            Self::ST(v, p) => {
                cost = 10.0 * v.rel_val() as f64;
                if p.modifiers.contains_key(&Modifier::NoFineManipulators) {
                    cost *= 0.6
                }
                if let Some(m) = p.modifiers.get(&Modifier::Size) {
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
            Self::DX(v, p) => Self::DX(v + rhs, p),
            Self::HT(v, p) => Self::HT(v + rhs, p),
            Self::IQ(v, p) => Self::IQ(v + rhs, p),
            Self::ST(v, p) => Self::ST(v + rhs, p),
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
            Self::DX(v, p) => Self::DX(v - rhs, p),
            Self::HT(v, p) => Self::HT(v - rhs, p),
            Self::IQ(v, p) => Self::IQ(v - rhs, p),
            Self::ST(v, p) => Self::ST(v - rhs, p),
        }
    }
}

impl AddAssign<i32> for Attribute {
    /**
     `self += i32`
     */
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
    /**
     `self += i32`
     */
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
    use crate::{attrib::AttributeValued, misc::{approx::Approx, costly::Costly}, modifier::{Modifier, ModifierValue}};

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
        let a = Attribute::default(AttributeType::DX);
        let a = a - 2;
        assert_eq!(-2, a.rel_val());
        assert_eq!(-40.0, a.cost());
    }

    #[test]
    fn rel_val_clamping_works() {
        let a = Attribute::default(AttributeType::DX);
        let a = a - 10;
        assert_eq!(-9, a.rel_val());
        assert_eq!(-180.0, a.cost());
    }

    #[test]
    fn preset_modifier_works() {
        let mut a = Attribute::default(AttributeType::ST);
        a.set_modifier((Modifier::NoFineManipulators, None));
        a += 2;
        assert_eq!(2, a.rel_val());
        assert_eq!(12.0, a.cost());
    }

    #[test]
    fn set_modifier_chaining_works() {
        let mut a = Attribute::default(AttributeType::ST);
        a   .set_modifier((Modifier::NoFineManipulators, None))
            .set_modifier((Modifier::Size, Some(ModifierValue::I(-2))));
        a += 2;

        // We can't use assert_eq!() because of (potential) float imprecision.
        assert!(a.cost().approx(9.6));
    }
}
