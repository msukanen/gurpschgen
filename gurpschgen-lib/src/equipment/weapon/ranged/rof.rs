use regex::Captures;

use crate::equipment::weapon::ranged::RX_R_ROF;

/**
 Rate of Fire (RoF).
 */
#[derive(Debug, Clone, PartialEq)]
pub enum RoF {
    /// **X*** → auto &ndash; e.g. SMGs, LMGs, etc.
    FullAuto(i32),
    /// **X~** → semi-auto &ndash; e.g. Colt 1911
    SemiAuto(i32),
    /// **Skill/X** → RoF based on skill's divisor.
    Skill(i32),
    /// **1/X** → multiple seconds to reload &ndash; blunderbus, etc.
    Slow(i32, i32),
    /// **X** → 6-shooters, etc.
    Trigger(i32),
}

impl From<Captures<'_>> for RoF {
    fn from(value: Captures<'_>) -> Self {
        let x = value.name("rof").unwrap().as_str();
        if let Some(n) = value.name("rof1") {
            let n = n.as_str().parse::<i32>().unwrap();
            if x.contains("~") {
                Self::SemiAuto(n)
            } else if x.contains("*") {
                Self::FullAuto(n)
            } else if x.contains("/") {
                Self::Slow(n, value.name("rof2").unwrap().as_str().parse::<i32>().unwrap())
            } else {
                if n < 4 {
                    Self::Trigger(n)
                } else {
                    Self::FullAuto(n)
                }
            }
        } else if x.contains("/") {
            Self::Skill(value.name("rof2").unwrap().as_str().parse::<i32>().unwrap())
        } else {
            todo!("Something gone wrong with: {x}")
        }
    }
}

impl From<&str> for RoF {
    fn from(value: &str) -> Self {
        if let Some(x) = RX_R_ROF.captures(value) {
            Self::from(x)
        } else {
            panic!("FATAL: \"{value}\" cannot be translated into RoF-value.")
        }
    }
}
