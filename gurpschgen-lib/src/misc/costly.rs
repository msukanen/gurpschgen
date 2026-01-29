//! Some things cost something…
/// A trait for anything with a (generic) cost of some sort (be it credits or points).
pub trait HasCost {
    /// Get cost.
    /// 
    /// Usually this is either *point cost* or *$cost*.
    /// 
    /// # Returns
    /// …are costly ;-)
    fn cost(&self) -> f64;
}
