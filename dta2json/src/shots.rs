use gurpschgen_lib::equipment::weapon::ranged::shots::{Battery, Shots};
use regex::Captures;

use crate::ranged::RX_R_SHOTS;

pub(crate) fn shots_from_captures(value: Captures<'_>) -> Shots {
    if let Some(x) = value.name("fthrow1") {
        let x = x.as_str().parse::<i32>().unwrap();
        let y = value.name("fthrow2").unwrap().as_str().parse::<i32>().unwrap();
        Shots::Flamethrowerlike(x, y)
    }
    else if let Some(x) = value.name("splus") {
        let x = x.as_str().parse::<i32>().unwrap();
        if let Some(y) = value.name("splusmod") {
            let Ok(y) = y.as_str().parse::<i32>() else { return Shots::Magazine(x) };
            Shots::MagazinePlus(x, y)
        } else {
            match x {
                ..=1 => Shots::Single,
                2 => Shots::DualBarrel,
                n => Shots::Magazine(n)
            }
        }
    } else if let Some(x) = value.name("battch") {
        let x = x.as_str().parse::<i32>().unwrap();
        Shots::Battery(x, Battery::from(value.name("batt").unwrap().as_str()))
    } else if let Some(_) = value.name("xxxbelt") {
        Shots::Belt(i32::MAX)
    } else if let Some(x) = value.name("bfed") {
        let x = x.as_str().parse::<i32>().unwrap();
        if let Some(_) = value.name("boxfed") {
            Shots::Box(x)
        } else {
            Shots::Belt(x)
        }
    } else {
        panic!("")
    }
}

pub(crate) fn shots_from_tuple(value: (&str, &str)) -> Shots {
    if let Some(x) = RX_R_SHOTS.captures(value.1) {
        shots_from_captures(x)
    } else {
        panic!("FATAL: \"{}\" does not conform with any known Shots model!", value.1)
    }
}

#[cfg(test)]
mod shots_tests {
    use gurpschgen_lib::equipment::weapon::ranged::shots::{Battery, Shots};

    use crate::shots::shots_from_tuple;

    #[test]
    fn shots_num_plus_num_works() {
        let data = ("Glock 20 10mm", "Shots 15+1");
        assert_eq!(Shots::MagazinePlus(15, 1), shots_from_tuple(data));
    }

    #[test]
    fn shots_bracednum_num_works() {
        let data = ("LPO-50", "Shots (3)9");
        assert_eq!(Shots::Flamethrowerlike(3, 9), shots_from_tuple(data));
    }

    #[test]
    fn shots_xxxb_works() {
        let data = ("Mk19 AGL 40x53mm", "Shots xxxB");
        assert_eq!(Shots::Belt(i32::MAX), shots_from_tuple(data));
    }

    #[test]
    fn shots_battery_works() {
        let data = ("H-90 Rifle", "Shots 200/D");
        assert_eq!(Shots::Battery(200, Battery::D), shots_from_tuple(data));
    }

    #[test]
    fn shots_belt_works() {
        let data = ("M60 7.62x51mm", "Shots 100B");
        assert_eq!(Shots::Belt(100), shots_from_tuple(data));
    }

    #[test]
    fn shots_unadorned_num_works() {
        let data = ("NSV 12.7x108mm", "Shots 50");
        assert_eq!(Shots::Magazine(50), shots_from_tuple(data));
    }

    #[test]
    fn shots_box_works() {
        let data = ("EX34 Chain Gun 7.62x51mm", "Shots 500Box");
        assert_eq!(Shots::Box(500), shots_from_tuple(data))
    }
}
