/**
 A trait for anything which might have weight/mass.
 */
pub trait Weighed {
    /**
     Get weight (or lack of such).
     
     It's up to the game rules to dictate the *unit* itself.

     **Returns** *unit-agnostic* value &ndash; or `None`.
     */
    fn weight(&self) -> Option<f64>;
}
