use std::collections::HashMap;

use crate::equipment::weapon::{RX_DMGD, ranged::RX_R_SPEC_DMG};

/**
 Damage types.
 */
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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

impl From<i32> for DamageResistance {
    fn from(value: i32) -> Self {
        Self::All(value)
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

impl From<i32> for PassiveDefense {
    fn from(value: i32) -> Self {
        Self::All(value)
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
    /// **Var**iable damage, see your games' rules for details.
    Var(DamageDelivery),
    /// **Spec**iable damage, see your games' rules for details.
    Spec(DamageDelivery),
}

/**
 Some common damage delivery methods.
 */
#[derive(Debug, Clone, PartialEq)]
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

impl From<&str> for Damage {
    fn from(value: &str) -> Self {
        //
        // Let's attempt to deal with damage...
        //
        if let Some(caps) = RX_R_SPEC_DMG.with(|rx| rx.captures(value)) {
            let dmgvar = if let Some(x) = caps.name("specvar") {
                DamageDelivery::Dice(x.as_str().parse::<i32>().unwrap(), 0)
            } else {
                DamageDelivery::Var
            };
            Self::from((DamageType::Spec, dmgvar))
        } else if let Some(caps) = RX_DMGD.with(|rx| rx.captures(value)) {
            let dmgtype = match caps.name("dtype").unwrap().as_str() {
                "cut" |
                "Cut" => DamageType::Cut,
                "cr"  |
                "Cr"  => DamageType::Cr,
                "imp" |
                "Imp" => DamageType::Imp,
                "var" |
                "Var" => DamageType::Var,
                n => todo!("dtype \"{n}\" not implemented!")
            };
            
            // Deal with delivery method, if present:
            if let Some(mode) = caps.name("ddel") {
                // .. get ±# modifier, if any:
                let modifier = if let Some(modifier) = caps.name("dmod") {
                    modifier.as_str().parse::<i32>().unwrap()
                } else {0};
                
                match mode.as_str() {
                    "sw"  |
                    "Sw"  => Self::from((dmgtype, DamageDelivery::Sw(modifier))),
                    "thr" |
                    "Thr" => Self::from((dmgtype, DamageDelivery::Thr(modifier))),
                    "var" |
                    "Var" => Self::from((dmgtype, DamageDelivery::Var)),
                    "spec"|
                    "Spec"=> Self::from((dmgtype, DamageDelivery::Spec(modifier))),
                    n => todo!("ddel \"{n}\" not implemented!")
                }
            }
            // .. d6 representation:
            else if let Some(_) = caps.name("d6") {
                if let Some(modifier) = caps.name("d6m") {
                    Self::from((dmgtype, DamageDelivery::Dice(1, modifier.as_str().parse::<i32>().unwrap())))
                } else {
                    Self::from((dmgtype, DamageDelivery::Dice(1, 0)))
                }
            }
            // .. or deal with dmg dice representation, if present:
            else if let Some(dice) = caps.name("dd") {
                let dice = dice.as_str().parse::<i32>().unwrap();
                if let Some(modifier) = caps.name("ddm") {
                    let modifier = modifier.as_str().parse::<i32>().unwrap();
                    if let Some(dmul) = caps.name("dmul") {
                        Self::from((dmgtype, DamageDelivery::DiceMul(dice, modifier, dmul.as_str().parse::<f64>().unwrap())))
                    } else {
                        Self::from((dmgtype, DamageDelivery::Dice(dice, modifier)))
                    }
                } else if let Some(_) = caps.name("maybed") {
                    Self::from((dmgtype, DamageDelivery::Dice(dice, 0)))
                } else {
                    Self::from((dmgtype, DamageDelivery::Flat(dice)))
                }
            }
            // .. or :-( bugger...?!
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
            DamageType::Imp => Self::Imp(value.1),
            DamageType::Var => Self::Var(value.1),
            DamageType::Spec => Self::Spec(value.1),
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
    fn cut_10d_works() {
        let data = "Cut/10d";
        let dmg = Damage::from(data);
        assert_eq!(Damage::Cut(DamageDelivery::Dice(10, 0)), dmg);
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
