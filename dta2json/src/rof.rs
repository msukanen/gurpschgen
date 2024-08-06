use gurpschgen_lib::equipment::weapon::ranged::rof::RoF;
use regex::Captures;

use crate::ranged::RX_R_ROF;

pub(crate) fn rof_from_captures(value: Captures<'_>) -> RoF {
    let x = value.name("rof").unwrap().as_str();
    if let Some(n) = value.name("rof1") {
        let n = n.as_str().parse::<i32>().unwrap();
        if x.contains("~") {
            RoF::SemiAuto(n)
        } else if x.contains("*") {
            RoF::FullAuto(n)
        } else if x.contains("/") {
            RoF::Slow(n, value.name("rof2").unwrap().as_str().parse::<i32>().unwrap())
        } else {
            if n < 4 {
                RoF::Trigger(n)
            } else {
                RoF::FullAuto(n)
            }
        }
    } else if x.contains("/") {
        RoF::Skill(value.name("rof2").unwrap().as_str().parse::<i32>().unwrap())
    } else {
        todo!("Something gone wrong with: {x}")
    }
}

pub(crate) fn rof_from_str(value: &str) -> RoF {
    if let Some(x) = RX_R_ROF.captures(value) {
        rof_from_captures(x)
    } else {
        panic!("FATAL: \"{value}\" cannot be translated into RoF-value.")
    }
}
