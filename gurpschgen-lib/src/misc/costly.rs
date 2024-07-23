/**
 A trait for anything with a cost.
 */
pub trait Costly {
    /**
     Get cost.
     
     Usually this is either *point cost* or *$cost*.

     **Returns** something costly ;-)
     */
    fn cost(&self) -> f64;
}
