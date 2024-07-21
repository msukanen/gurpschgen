use crate::misc::costly::Costly;

#[derive(Debug, Clone)]
pub struct Armor {
    dr: i32,
    pd: i32,
    cost: f64,
    weight: f64,
}

impl Costly for Armor {
    fn cost(&self) -> f64 {
        self.cost
    }
}

impl From<(&str, &str)> for Armor {
    fn from(value: (&str, &str)) -> Self {
        todo!("")
    }
}
