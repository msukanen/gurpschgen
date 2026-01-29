use crate::{edition::Edition, misc::tl::TL};

/// Some generic configuration basics…
pub struct Config {
    /// GURPS edition to use.
    pub edition: Edition,
    /// Female gender is considered a disadvantage? Before TL7 this is often the case…
    pub female_as_5pts_disadvantage: bool,
    /// Avg. tech level.
    pub tl: TL,
}

impl Default for Config {
    /// Get typical TL3 default.
    fn default() -> Self {
        Self { edition: Edition::default(), female_as_5pts_disadvantage: true, tl: 3.into() }
    }
}

impl Config {
    /// Get typical TL7 default.
    pub fn default_tl7(edition: Edition) -> Self {
        Self { edition, female_as_5pts_disadvantage: false, tl: 7.into(), }
    }

    /// Get typical TL8 default.
    pub fn default_tl8(edition: Edition) -> Self {
        Self { edition, female_as_5pts_disadvantage: false, tl: 8.into(), }
    }
}
