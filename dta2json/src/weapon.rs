use gurpschgen_lib::equipment::weapon::Weapon;
use once_cell::sync::Lazy;

use crate::{melee::melee_wpn_from_tuple, ranged::ranged_wpn_from_tuple};

pub(crate) static RX_SIMPLE_ANY_WPN: Lazy<regex::Regex> = Lazy::new(||regex::Regex::new(r"^(?:\s*(?:[cC](?:ut|r)|[iI]mp|[vV]ar|[sS]pec)[^;,]*,)").unwrap());
static RX_SIMPLE_RANGED: Lazy<fancy_regex::Regex> = Lazy::new(||fancy_regex::Regex::new(r"(?:(?:[sS]pec?|(?:(?:[cC]r|[cC]ut|[iI]mp|[vV]ar)\/(?![sS]w|[tT]hr)))[^;]*(?:SS|Max|Rcl|(Acc;)))").unwrap());
pub(crate) static RX_DMGD: Lazy<regex::Regex> = Lazy::new(||regex::Regex::new(r"(?:\s*(?<dtype>[cC]ut|[cC]r|[iI]mp|[vV]ar)(?:\/|\s)((?:(?:(?<ddel>[sS]w|[tT]hr|Var)(?<dmod>[+-]\d+)?))|(?<d6>d6(?<d6m>[-+]\d+)?)|(?:(?<dd>\d+)(?<maybed>d)?(?:(?<ddm>[-+]\d+)(?:\([xX](?<dmul>\d+(?:[.]\d+)?)\))?)?)))").unwrap());
pub(crate) static RX_MAX_DMG: Lazy<regex::Regex> = Lazy::new(||regex::Regex::new(r"(?:\s*(?:[mM]aximum|M(?:ax|AX))?\s+(?:dmg|DMG|[dD]amage)\s+(?:(?<dmgd>\d+)[d]?(?<dmgb>[-+]\d+)?))").unwrap());

pub(crate) fn wpn_from_tuple(value: (&str, &str)) -> Weapon {
    if let Ok(Some(_)) = RX_SIMPLE_RANGED.captures(value.1) {
        #[cfg(test)] println!("Ranged: {}", value.0);
        Weapon::Ranged(ranged_wpn_from_tuple(value))
    } else {
        #[cfg(test)] println!("Melee: {}", value.0);
        Weapon::Melee(melee_wpn_from_tuple(value))
    }
}

#[cfg(test)]
mod weapons_tests {
    use gurpschgen_lib::{damage::{Damage, DamageDelivery}, equipment::weapon::Weapon};

    use crate::weapon::wpn_from_tuple;

    #[test]
    fn melee_classification_works() {
        let data = ("        Snotswod  ", "   Cut/Sw,Acc+1,ST7;  500,3.0  ;  Broadsword ;  It's absolutely horrible...; Sword Quality, Weapon, Melee Weapon");
        let wpn = wpn_from_tuple(data);
        assert!(match wpn {
            Weapon::Melee(_) => true,
            _ => false
        })
    }

    #[test]
    fn ranged_classification_works() {
        let data = ("  Laz0r Pistol  ", " Imp/1d, SS0;  100,2.0  ;  Guns: Pistol ;  High IQ Bonus; ");
        let wpn = wpn_from_tuple(data);
        assert!(match wpn {
            Weapon::Ranged(r) => {
                println!("{:?}", r.damage());
                assert!(r.damage().contains(&Damage::Imp(DamageDelivery::Dice(1, 0))));
                true
            },
            _ => false
        })
    }
}
