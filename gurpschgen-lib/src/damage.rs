use std::collections::HashMap;

use crate::RX_DMGD;

/**
 Damage types.
 */
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum DamageType {
    Cut,
    Cr,
    Energy,// anything what "cauterizes" the wound instantly.
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
#[derive(Debug, Clone, PartialEq)]
pub enum Damage {
    Cut(DamageDelivery),
    Cr(DamageDelivery),
    Energy(DamageDelivery),
    Imp(DamageDelivery),
}

/**
 Some common damage delivery methods.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum DamageDelivery {
    /**
     **Dice** & modifier. E.g. guns and other weapons that have relatively stable/fixed dmg model.
     */
    Dice(i32, i32),
    /**
     About same as [Dice(x,y)][DamageDelivery::Dice] but with a multiplier for final delivered dmg.
     */
    DiceMul(i32, i32, f64),
    /**
     **Flat** dmg without any variation whatsoever.
     */
    Flat(i32),
    /**
     **Sw**ing based on ST; embedded modifier.
     */
    Sw(i32),
    /**
     **Thr**ust based on ST; embedded modifier.
     */
    Thr(i32),
}

impl From<&str> for Damage {
    fn from(value: &str) -> Self {
        //
        // Let's attempt to deal with damage...
        //
        if let Some(caps) = RX_DMGD.with(|rx| rx.captures(value)) {
            let dmgtype = match caps.name("dmgtype").unwrap().as_str() {
                "Cut" => DamageType::Cut,
                "Cr" => DamageType::Cr,
                "Imp" => DamageType::Imp,
                n => panic!("FATAL: unrecognized damage type \"{n}\"")
            };
            // Does input conform with e.g. Cut/Sw±# pattern?
            if let Some(mode) = caps.name("dmgdt") {
                if let Some(modifier) = caps.name("dmgdtm") {
                    let modifier = modifier.as_str().parse::<i32>().unwrap();
                    match mode.as_str() {
                        "Sw" => Self::from((dmgtype, DamageDelivery::Sw(modifier))),
                        "Thr" => Self::from((dmgtype, DamageDelivery::Thr(modifier))),
                        n => todo!("Damage type {n} not (yet?) implemented!")
                    }
                } else {
                    panic!("FATAL")
                }
            }
            // Does input conform with e.g. Imp/#±# pattern?
            else if let Some(dice) = caps.name("dmgd") {
                let dice = dice.as_str().parse::<i32>().unwrap();
                if let Some(modifier) = caps.name("dmgdm") {
                    Self::from((dmgtype, DamageDelivery::Dice(dice, modifier.as_str().parse::<i32>().unwrap())))
                } else {
                    Self::from((dmgtype, DamageDelivery::Flat(dice)))
                }
            }
            // :-( bugger...?!
            else {
                panic!("FATAL: malformed DTA \"{value}\"")
            }
        }
        //
        // Utterly unknown dmg model?!
        //
        else {
            todo!("What to do with \"{value}\"?!")
        }
    }
}

impl From<(DamageType, DamageDelivery)> for Damage {
    /**
     Construct [Damage] from `value`.
     */
    fn from(value: (DamageType, DamageDelivery)) -> Self {
        match value.0 {
            DamageType::Cr => Self::Cr(value.1),
            DamageType::Cut => Self::Cut(value.1),
            DamageType::Energy => Self::Energy(value.1),
            DamageType::Imp => Self::Imp(value.1)
        }
    }
}

#[cfg(test)]
mod damage_tests {
    use super::{Damage, DamageDelivery};

    #[test]
    fn cr_sw_works() {
        let data = "Cr/Sw+2";
        let dmg = Damage::from(data);
        assert_eq!(Damage::Cr(DamageDelivery::Sw(2)), dmg);
    }

    #[test]
    fn cut_thr_works() {
        let data = "Cut/Thr-1";
        let dmg = Damage::from(data);
        assert_eq!(Damage::Cut(DamageDelivery::Thr(-1)), dmg);
    }

    #[test]
    fn imp_dice_works() {
        let data = "Imp/1d-2";
        let dmg = Damage::from(data);
        assert_eq!(Damage::Imp(DamageDelivery::Dice(1, -2)), dmg);
    }

    #[test]
    fn cut_flatdmg_works() {
        let data = "Cut/10";
        let dmg = Damage::from(data);
        assert_eq!(Damage::Cut(DamageDelivery::Flat(10)), dmg);
    }

    #[test]
    fn cr_dice_works() {
        let data = "Cr/2+1";
        let dmg = Damage::from(data);
        assert_eq!(Damage::Cr(DamageDelivery::Dice(2, 1)), dmg);
    }

    #[test]
    #[should_panic]
    fn crx_dice_fails() {
        let data = "Crx/66+6";
        let dmg = Damage::from(data);
        assert_eq!(Damage::Cr(DamageDelivery::Flat(6)), dmg);
    }
}
