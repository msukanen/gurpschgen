use std::{collections::HashMap, ops::{Add, Sub}, cmp::max};

use crate::modifier::{Modifier, ModifierValue};

use super::Attribute;

pub struct ST {
    base_val: i32,
    rel_val: i32,
    modifiers: HashMap<Modifier, ModifierValue>
}

impl Attribute for ST {
    fn base_value(&self) -> i32 {
        self.base_val
    }

    fn cost(&self) -> f64 {
        let mut cost = 10.0 * self.rel_value() as f64;
        
        if let Some(x) = self.modifiers.get(&Modifier::Size) {
            cost *= 1.0 + 0.01 * (10 * match x {
                ModifierValue::I(x) => max(-8, *x),
                _ => 0
            }) as f64
        }

        if let Some(_) = self.modifiers.get(&Modifier::NoFineManipulators) {
            cost *= 0.6
        }

        cost
    }

    fn rel_value(&self) -> i32 {
        self.rel_val
    }

    fn value(&self) -> i32 {
        self.base_value() + self.rel_value()
    }
}

impl ST {
    /**
     Instantiate new `ST` with desired values.

     **Params**
     * `base_val` - base value, obviously.
     * `rel_val` - relative value, in relation to `base_val`.
     * `modifiers` - hash map of applied [Modifier], if any.
     
     **Returns** `ST`.
     */
    pub fn new(base_val: i32, rel_val: i32, modifiers: Option<HashMap<Modifier, ModifierValue>>) -> Self {
        Self {
            base_val,
            rel_val,
            modifiers: if let Some(m) = modifiers {m} else {HashMap::new()},
        }
    }

    /**
     Instantiate `ST` with default values.

     **Returns** `ST`.
     */
    pub fn default() -> Self {
        Self {
            base_val: 10,
            rel_val: 0,
            modifiers: HashMap::new(),
        }
    }
}

impl Add<i32> for ST {
    type Output = Self;
    /**
     Add `rhs` to relative value of the attribute.
     */
    fn add(self, rhs: i32) -> Self::Output {
        Self {
            base_val: self.base_val,
            rel_val: self.rel_val + rhs,
            modifiers: self.modifiers,
        }
    }
}

impl Sub<i32> for ST {
    type Output = Self;
    /**
     Subtract `rhs` from relative value of the attribute.
     */
    fn sub(self, rhs: i32) -> Self::Output {
        self + (-rhs)
    }
}

#[cfg(test)]
mod attrib_st_tests {
    use std::collections::HashMap;

    use crate::{attrib::Attribute, modifier::{Modifier, ModifierValue}};

    use super::ST;

    #[test]
    fn base_st_works() {
        let st = ST::default();
        assert_eq!(10, st.base_value());
        assert_eq!(0, st.rel_value());
        assert_eq!(10, st.value());
    }

    #[test]
    fn st_with_nofinemanipulators_works() {
        let mut st = ST::new(10, 1, Some({
            let mut m = HashMap::new();
            m.insert(Modifier::NoFineManipulators, ModifierValue::Ignore);
            m
        }));
        // ST11 = 10, -40% = 6.0
        assert_eq!(6.0, st.cost());
        
        // ST12 = 20, -40% = 12.0
        st.rel_val += 1;
        assert_eq!(12.0, st.cost());

        // ST8 = -20, -40% = -12.0
        st.rel_val = -2;
        assert_eq!(-12.0, st.cost());
    }

    #[test]
    fn st_with_size_modifier_works() {
        let mut st = ST::new(10, 0, Some({
            let mut m = HashMap::new();
            // Let's use -2 as size modifier.
            m.insert(Modifier::Size, ModifierValue::I(-2));
            m
        }));
        // ST10 = 0
        assert_eq!(0.0, st.cost());

        // ST11 = 10, 10%×-2 = 8.0
        st.rel_val += 1;
        assert_eq!(8.0, st.cost());
    }
}
