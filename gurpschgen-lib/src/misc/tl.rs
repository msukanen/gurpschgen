use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum TL {
    Exact(i32),
    About { default: i32, min: i32, max: i32 },
}

#[cfg(test)]
mod tl_tests {
    use super::TL;

    #[test]
    fn tl_exact_works() {
        let tl = TL::Exact(8);
        let json = serde_json::to_string(&tl).unwrap();
        println!("{json}");
    }
}
