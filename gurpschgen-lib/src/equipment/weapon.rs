use crate::RX_SIMPLE_RANGED;
use melee::Melee;
use ranged::Ranged;

use crate::misc::costly::Costly;

pub mod melee;
pub mod ranged;

#[derive(Debug, Clone)]
pub enum Weapon {
    Melee(Melee),
    Ranged(Ranged),
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
        RX_SIMPLE_RANGED.with(|rx| if let Some(_) = rx.captures(value.1) {
            Self::Ranged(Ranged::from(value))
        } else {
            Self::Melee(Melee::from(value))
        })
    }
}

#[cfg(test)]
mod weapons_tests {
    use super::Weapon;

    #[test]
    fn melee_works() {
        let data = ("        Broadsword  ", "   Cut/Sw+1, Cr/Thr+1, Imp/Sw+3, Cut/Thr-2;  500,3.0  ;  Broadsword ;  It's absolutely horrible...; Sword Quality, Weapon, Melee Weapon");
        let wpn = Weapon::from(data);
        assert!(match wpn {
            Weapon::Melee(_) => true,
            _ => false
        })
    }

    #[test]
    fn ranged_works() {
        let data = ("  Laz0r Pistol  ", " Imp/1d, SS0;  100,2.0  ;  Guns: Pistol ;  High IQ Bonus; ");
        let wpn = Weapon::from(data);
        assert!(match wpn {
            Weapon::Ranged(_) => true,
            _ => false
        })
    }
}
