use super::costly::HasCost;

/**
 A trait for anything with levels/ranks.
 */
pub trait Leveled: HasCost {
    /**
     Get current level/rank.
     */
    fn level(&self) -> usize;
    /**
     Get max level/rank, if applicable.

     **Returns** either some `usize` value or `None`.
     */
    fn max_level(&self) -> Option<usize> { None }
}
