use melee::Melee;
use ranged::Ranged;

use crate::misc::{costly::Costly, st_req::STRequired};

thread_local! {
    pub(crate) static RX_SIMPLE_ANY_WPN: regex::Regex = regex::Regex::new(r"^(?:\s*(?:[cC](?:ut|r)|[iI]mp|[vV]ar|[sS]pec)[^;,]*,)").unwrap();
    static RX_SIMPLE_RANGED: fancy_regex::Regex = fancy_regex::Regex::new(r"(?:(?:[sS]pec?|(?:(?:[cC]r|[cC]ut|[iI]mp|[vV]ar)\/(?![sS]w|[tT]hr)))[^;]*(?:SS|Max|Rcl|(Acc;)))").unwrap();
    pub(crate) static RX_DMGD: regex::Regex = regex::Regex::new(r"(?:\s*(?<dtype>[cC]ut|[cC]r|[iI]mp|[vV]ar)\/((?:(?:(?<ddel>[sS]w|[tT]hr|Var)(?<dmod>[+-]\d+)?))|(?:(?<dd>\d+)(?<maybed>d)?(?:(?<ddm>[-+]\d+)(?:\([xX](?<dmul>\d+(?:[.]\d+)?)\))?)?)))").unwrap();
}

pub mod melee;
pub mod ranged;

#[derive(Debug, Clone)]
pub enum Weapon {
    Melee(Melee),
    Ranged(Ranged),
}

impl STRequired for Weapon {
    fn st_req(&self) -> &Option<i32> {
        match self {
            Self::Melee(x) => x.st_req(),
            Self::Ranged(x) => x.st_req(),
        }
    }
}

impl Costly for Weapon {
    fn cost(&self) -> f64 {
        match self {
            Self::Melee(a) => a.cost(),
            Self::Ranged(a) => a.cost(),
        }
    }
}

impl From<(&str, &str)> for Weapon {
    fn from(value: (&str, &str)) -> Self {
        if let Ok(Some(_)) = RX_SIMPLE_RANGED.with(|rx| rx.captures(value.1)) {
            Self::Ranged(Ranged::from(value))
        } else {
            Self::Melee(Melee::from(value))
        }
    }
}

#[cfg(test)]
mod weapons_tests {
    use crate::damage::{Damage, DamageDelivery};

    use super::Weapon;

    #[test]
    fn melee_classification_works() {
        let data = ("        Snotswod  ", "   Cut/Sw,Acc+1,ST7;  500,3.0  ;  Broadsword ;  It's absolutely horrible...; Sword Quality, Weapon, Melee Weapon");
        let wpn = Weapon::from(data);
        assert!(match wpn {
            Weapon::Melee(_) => true,
            _ => false
        })
    }

    #[test]
    fn ranged_classification_works() {
        let data = ("  Laz0r Pistol  ", " Imp/1d, SS0;  100,2.0  ;  Guns: Pistol ;  High IQ Bonus; ");
        let wpn = Weapon::from(data);
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
