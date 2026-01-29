//! Some GURPS edition-specific specs.
pub enum Edition {
    Ed3,
    Ed4,
}

impl Default for Edition {
    /// Get default edition.
    /// 
    /// Most legacy data is for [3rd Ed][Edition::Ed3], and thus we'll default to that.
    fn default() -> Self {
        Self::Ed3
    }
}
