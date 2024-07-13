pub mod st;

/**
 Trait for all sorts of attributes/stats.
 */
pub trait Attribute {
    /**
     Get base/root value.

     **Returns** `i32`.
     */
    fn base_value(&self) -> i32;
    /**
     Get relative value.

     **Returns** `i32`.
     */
    fn rel_value(&self) -> i32;
    /**
     Get effective value.

     **Returns** `i32`.
     */
    fn value(&self) -> i32;
    /**
     Get point cost.

     **Returns** `f64`.
     */
    fn cost(&self) -> f64;
}
