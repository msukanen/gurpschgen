use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum TL {
    Exact(u8),
    About { default: u8, min: u8, max: u8 },
}

const fn minmax_tl(tl: i32) -> TL {
    match tl {
        ..=0 => TL::Exact(0),
        ..=15 => TL::Exact(tl as u8),
        _ => TL::Exact(16)
    }
}

impl From<i32> for TL {
    fn from(value: i32) -> Self {
        minmax_tl(value)
    }
}

impl From<u8> for TL {
    fn from(value: u8) -> Self {
        minmax_tl(value as i32)
    }
}

#[cfg(test)]
mod tl_tests {
    use super::TL;

    #[test]
    fn tl_exact_works() {
        let tl = TL::Exact(8);
        let json = serde_json::to_string(&tl).unwrap();
        let tl: TL = serde_json::from_str(&json).unwrap();
        assert_eq!(TL::Exact(8), tl);
    }
}
