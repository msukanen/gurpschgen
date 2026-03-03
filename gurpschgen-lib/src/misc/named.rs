//! Name-based stuff.
/// A trait for anything with a name.
pub trait HasName {
    /// Get the name of something or other.
    fn name(&self) -> &str;
}
