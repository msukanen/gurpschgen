use regex::Captures;

use crate::equipment::weapon::ranged::RX_R_SHOTS;

/**
 Various high-tech energy battery types.
 */
#[derive(Debug, Clone, PartialEq)]
pub enum Battery {
    AAA, AA, A, B, C, D, E, F,
}

impl From<&str> for Battery {
    fn from(value: &str) -> Self {
        match value {
            "AAA" => Self::AAA,
            "AA" => Self::AA,
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "D" => Self::D,
            "E" => Self::E,
            "F" => Self::F,
            n => todo!("FATAL: battery type \"{n}\" not implemented?!")
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Shots {
    Battery(i32, Battery),
    Belt(i32),
    Box(i32),
    /// Generally 2×barrel shotgun.
    DualBarrel,
    Flamethrowerlike(i32, i32),
    Magazine(i32),
    MagazinePlus(i32, i32),
    /// 1×barrel shotgun, bolt-action rifle, one-shot Derringer, etc.
    Single,
}

impl From<Captures<'_>> for Shots {
    fn from(value: Captures<'_>) -> Self {
        if let Some(x) = value.name("fthrow1") {
            let x = x.as_str().parse::<i32>().unwrap();
            let y = value.name("fthrow2").unwrap().as_str().parse::<i32>().unwrap();
            Self::Flamethrowerlike(x, y)
        }
        else if let Some(x) = value.name("splus") {
            let x = x.as_str().parse::<i32>().unwrap();
            if let Some(y) = value.name("splusmod") {
                let Ok(y) = y.as_str().parse::<i32>() else { return Self::Magazine(x) };
                Self::MagazinePlus(x, y)
            } else {
                match x {
                    ..=1 => Self::Single,
                    2 => Self::DualBarrel,
                    n => Self::Magazine(n)
                }
            }
        } else if let Some(x) = value.name("battch") {
            let x = x.as_str().parse::<i32>().unwrap();
            Self::Battery(x, Battery::from(value.name("batt").unwrap().as_str()))
        } else if let Some(_) = value.name("xxxbelt") {
            Self::Belt(i32::MAX)
        } else if let Some(x) = value.name("bfed") {
            let x = x.as_str().parse::<i32>().unwrap();
            if let Some(_) = value.name("boxfed") {
                Self::Box(x)
            } else {
                Self::Belt(x)
            }
        } else {
            panic!("")
        }
    }
}

impl From<(&str, &str)> for Shots {
    fn from(value: (&str, &str)) -> Self {
        if let Some(x) = RX_R_SHOTS.captures(value.1) {
            Self::from(x)
        } else {
            panic!("FATAL: \"{}\" does not conform with any known Shots model!", value.1)
        }
    }
}

#[cfg(test)]
mod shots_tests {
    use crate::equipment::weapon::ranged::shots::Battery;

    use super::Shots;

    #[test]
    fn shots_num_plus_num_works() {
        let data = ("Glock 20 10mm", "Shots 15+1");
        assert_eq!(Shots::MagazinePlus(15, 1), Shots::from(data));
    }

    #[test]
    fn shots_bracednum_num_works() {
        let data = ("LPO-50", "Shots (3)9");
        assert_eq!(Shots::Flamethrowerlike(3, 9), Shots::from(data));
    }

    #[test]
    fn shots_xxxb_works() {
        let data = ("Mk19 AGL 40x53mm", "Shots xxxB");
        assert_eq!(Shots::Belt(i32::MAX), Shots::from(data));
    }

    #[test]
    fn shots_battery_works() {
        let data = ("H-90 Rifle", "Shots 200/D");
        assert_eq!(Shots::Battery(200, Battery::D), Shots::from(data));
    }

    #[test]
    fn shots_belt_works() {
        let data = ("M60 7.62x51mm", "Shots 100B");
        assert_eq!(Shots::Belt(100), Shots::from(data));
    }

    #[test]
    fn shots_unadorned_num_works() {
        let data = ("NSV 12.7x108mm", "Shots 50");
        assert_eq!(Shots::Magazine(50), Shots::from(data));
    }

    #[test]
    fn shots_box_works() {
        let data = ("EX34 Chain Gun 7.62x51mm", "Shots 500Box");
        assert_eq!(Shots::Box(500), Shots::from(data))
    }
}
