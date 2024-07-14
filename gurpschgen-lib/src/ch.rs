use std::collections::HashMap;

use crate::{attrib::{Attribute, AttributeType, AttributeValued}, gender::Gender, misc::costly::Costly};

/**
 PC/NPC container.
 */
pub struct Ch {
    pub name: String,
    pub gender: Option<Gender>,
    pub st: Attribute,
    pub dx: Attribute,
    pub iq: Attribute,
    pub ht: Attribute,
    extra_hp: i32,
    extra_will: i32,
    extra_per: i32,
    extra_fp: i32,
    extra_speed: i32,
    extra_move: i32,
}

impl Ch {
    /**
     Instantiate a blank (or nearly blank) `Ch`.
     */
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            dx: Attribute::default(AttributeType::DX),
            ht: Attribute::default(AttributeType::HT),
            iq: Attribute::default(AttributeType::IQ),
            st: Attribute::default(AttributeType::ST),
            gender: None,// will be chosen later.
            extra_hp: 0,
            extra_will: 0,
            extra_per: 0,
            extra_fp: 0,
            extra_speed: 0,
            extra_move: 0,
        }
    }

    /**
     Get `Ch`'s **h**it **p**oints (HP).
     */
    pub fn hp(&self) -> i32 {
        self.st.value() + self.extra_hp
    }

    /**
     Get `Ch`'s **w**ill**p**ower (WP).
     */
    pub fn wp(&self) -> i32 {
        self.iq.value() + self.extra_will
    }

    /**
     Get `Ch`'s ***per**ception (Per).
     */
    pub fn per(&self) -> i32 {
        self.iq.value() + self.extra_per
    }

    /**
     Get `Ch`'s **f**atigue **p**oints (FP).
     */
    pub fn fp(&self) -> i32 {
        self.ht.value() + self.extra_fp
    }

    /**
     Get `Ch`'s basic **speed** score.
     */
    pub fn speed(&self) -> f64 {
        (self.ht.value() + self.dx.value() + self.extra_speed) as f64 / 4.0
    }

    /**
     Get `Ch`'s basic **move** score (yd/s).
     */
    // 'move' is a reserved word, so...: mov() instead.
    pub fn mov(&self) -> i32 {
        (self.speed() + self.extra_move as f64).trunc() as i32
    }
}

impl Costly for Ch {
    fn cost(&self) -> f64 {
          self.dx.cost()
        + self.ht.cost()
        + self.iq.cost()
        + self.st.cost()
        + 2.0 * self.extra_hp as f64
        + 5.0 * self.extra_will as f64
        + 5.0 * self.extra_per as f64
        + 3.0 * self.extra_fp as f64
    }
}

#[cfg(test)]
mod ch_tests {
    use super::Ch;

    #[test]
    fn init_works() {
        let ch = Ch::new("Nameless");
        assert_eq!("Nameless", ch.name);
        assert_eq!(10, ch.st);
    }

    #[test]
    fn speed_works() {
        let mut ch = Ch::new("Nameless");
        assert_eq!(5.0, ch.speed());
        assert_eq!(5, ch.mov());

        ch.dx += 2;
        assert_eq!(5.5, ch.speed());
        assert_eq!(5, ch.mov());

        ch.ht += 2;
        assert_eq!(6.0, ch.speed());
        assert_eq!(6, ch.mov());

        ch.extra_speed = 1;
        assert_eq!(6.25, ch.speed());
        
        ch.extra_move = 1;
        assert_eq!(7, ch.mov());
    }
}
