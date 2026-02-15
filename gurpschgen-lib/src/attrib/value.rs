use std::{cmp::max, ops::{Add, AddAssign, Sub, SubAssign}};

use serde::{Deserialize, Serialize};

pub const GURPS_STAT_BASE: i32 = 10;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttributeValue {
    pub(super) base_val: i32,
    pub(super) rel_val: i32,
}

pub trait HasAttributeValue {
    /// Get attribute's base/root value.
    fn base_val(&self) -> i32;

    /// Get attribute's relative (+/-) value.
    fn relative_value(&self) -> i32;

    /// Get attribute's effective value.
    /// 
    /// To change effective value, use `set_base_val()` or (in most cases) `set_rel_val()` respectively (if such exist).
    fn value(&self) -> i32 {
        self.base_val() + self.relative_value()
    }
}

impl Default for AttributeValue {
    fn default() -> Self {
        Self { base_val: GURPS_STAT_BASE, rel_val: 0 }
    }
}

impl AttributeValue {
    /// Set base value. Note that a `value` less than `1` will be treated as `1`.
    /// 
    /// # Args
    /// - `value`: new base value; ≤1 → 1
    pub fn set_base_val(&mut self, value: i32) -> &mut Self {
        self.base_val = max(1, value);
        self
    }
    
    /// Set relative value.
    /// 
    /// Relative value will be rejiggled if it'd bring effective value down to ≤0.
    /// 
    /// # Args
    /// - `value`: new relative value.
    pub fn set_rel_val(&mut self, value: i32) -> &mut Self {
        self.rel_val = max(-(self.base_val - 1), value);
        self
    }
}

impl HasAttributeValue for AttributeValue {
    fn base_val(&self) -> i32 { self.base_val }
    fn relative_value(&self) -> i32 { self.rel_val }
}

impl Add<i32> for AttributeValue {
    type Output = Self;
    /// Add `rhs` to *relative value* of `self`.
    fn add(self, rhs: i32) -> Self::Output {
        Self {
            rel_val: max(-(self.base_val - 1), self.rel_val + rhs),
            base_val: self.base_val,
        }
    }
}

impl Sub<i32> for AttributeValue {
    type Output = Self;
    /// Subtract `rhs` from *relative value* of `self`.
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
