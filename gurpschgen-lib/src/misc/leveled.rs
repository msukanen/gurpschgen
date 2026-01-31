//! Some things like Skills and Spells have "levels"…
use super::costly::HasCost;

/// A trait for anything with levels/ranks.
pub trait HasLevel: HasCost {
    /// Get current level/rank.
    fn level(&self) -> usize;

    /// Get max level/rank, if applicable.
    fn max_level(&self) -> Option<usize> { None }
}
