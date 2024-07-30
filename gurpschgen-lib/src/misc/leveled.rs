use crate::config::Config;

use super::costly::Costly;

/**
 A trait for anything with levels/ranks.
 */
pub trait Leveled: Costly {
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
