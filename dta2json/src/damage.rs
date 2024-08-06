use gurpschgen_lib::damage::{Damage, DamageDelivery, DamageType};

use crate::{ranged::RX_R_SPEC_DMG, weapon::RX_DMGD};

pub(crate) fn damage_from_str(value: &str) -> Damage {
    //
    // Let's attempt to deal with damage...
    //
    if let Some(caps) = RX_R_SPEC_DMG.captures(value) {
        let dmgvar = if let Some(x) = caps.name("specvar") {
            DamageDelivery::Dice(x.as_str().parse::<i32>().unwrap(), 0)
        } else {
            DamageDelivery::Var
        };
        damage_from_tuple((DamageType::Spec, dmgvar))
    } else if let Some(caps) = RX_DMGD.captures(value) {
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
                "Sw"  => damage_from_tuple((dmgtype, DamageDelivery::Sw(modifier))),
                "thr" |
                "Thr" => damage_from_tuple((dmgtype, DamageDelivery::Thr(modifier))),
                "var" |
                "Var" => damage_from_tuple((dmgtype, DamageDelivery::Var)),
                "spec"|
                "Spec"=> damage_from_tuple((dmgtype, DamageDelivery::Spec(modifier))),
                n => todo!("ddel \"{n}\" not implemented!")
            }
        }
        // .. d6 representation:
        else if let Some(_) = caps.name("d6") {
            if let Some(modifier) = caps.name("d6m") {
                damage_from_tuple((dmgtype, DamageDelivery::Dice(1, modifier.as_str().parse::<i32>().unwrap())))
            } else {
                damage_from_tuple((dmgtype, DamageDelivery::Dice(1, 0)))
            }
        }
        // .. or deal with dmg dice representation, if present:
        else if let Some(dice) = caps.name("dd") {
            let dice = dice.as_str().parse::<i32>().unwrap();
            if let Some(modifier) = caps.name("ddm") {
                let modifier = modifier.as_str().parse::<i32>().unwrap();
                if let Some(dmul) = caps.name("dmul") {
                    damage_from_tuple((dmgtype, DamageDelivery::DiceMul(dice, modifier, dmul.as_str().parse::<f64>().unwrap())))
                } else {
                    damage_from_tuple((dmgtype, DamageDelivery::Dice(dice, modifier)))
                }
            } else if let Some(_) = caps.name("maybed") {
                damage_from_tuple((dmgtype, DamageDelivery::Dice(dice, 0)))
            } else {
                damage_from_tuple((dmgtype, DamageDelivery::Flat(dice)))
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

/**
 Construct [Damage] from `value`.
 */
pub(crate) fn damage_from_tuple(value: (DamageType, DamageDelivery)) -> Damage {
    match value.0 {
        DamageType::Cr => Damage::Cr(value.1),
        DamageType::Cut => Damage::Cut(value.1),
        DamageType::Energy => Damage::Energy(value.1),
        DamageType::Imp => Damage::Imp(value.1),
        DamageType::Var => Damage::Var(value.1),
        DamageType::Spec => Damage::Spec(value.1),
    }
}

#[cfg(test)]
mod damage_tests {
    use crate::damage::damage_from_str;

    use super::{Damage, DamageDelivery};

    #[test]
    fn cr_sw_works() {
        let data = "Cr/Sw+2";
        let dmg = damage_from_str(data);
        assert_eq!(Damage::Cr(DamageDelivery::Sw(2)), dmg);
    }

    #[test]
    fn cut_thr_works() {
        let data = "Cut/Thr-1";
        let dmg = damage_from_str(data);
        assert_eq!(Damage::Cut(DamageDelivery::Thr(-1)), dmg);
    }

    #[test]
    fn imp_dice_works() {
        let data = "Imp/1d-2";
        let dmg = damage_from_str(data);
        assert_eq!(Damage::Imp(DamageDelivery::Dice(1, -2)), dmg);
    }

    #[test]
    fn cut_flatdmg_works() {
        let data = "Cut/10";
        let dmg = damage_from_str(data);
        assert_eq!(Damage::Cut(DamageDelivery::Flat(10)), dmg);
    }

    #[test]
    fn cut_10d_works() {
        let data = "Cut/10d";
        let dmg = damage_from_str(data);
        assert_eq!(Damage::Cut(DamageDelivery::Dice(10, 0)), dmg);
    }

    #[test]
    fn cr_dice_works() {
        let data = "Cr/2+1";
        let dmg = damage_from_str(data);
        assert_eq!(Damage::Cr(DamageDelivery::Dice(2, 1)), dmg);
    }

    #[test]
    #[should_panic]
    fn crx_dice_fails() {
        let data = "Crx/66+6";
        let dmg = damage_from_str(data);
        assert_eq!(Damage::Cr(DamageDelivery::Flat(6)), dmg);
    }
}
