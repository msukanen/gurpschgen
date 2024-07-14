/**
 A trait for anything with "point cost".
 */
pub trait Costly {
    /**
     Get point cost.
     */
    fn cost(&self) -> f64;
}
