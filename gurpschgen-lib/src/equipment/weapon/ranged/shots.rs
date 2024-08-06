use serde::{Deserialize, Serialize};

/**
 Various high-tech energy battery types.
 */
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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
