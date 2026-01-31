use crate::misc::TL;

/// Some generic configuration basics…
pub struct Config {
    /// Female gender is considered a disadvantage? Before TL7 this is often the case…
    pub female_as_5pts_disadvantage: bool,
    /// Avg. tech level.
    pub tl: TL,
}

impl Default for Config {
    /// Get typical TL3 default.
    fn default() -> Self {
        Self { female_as_5pts_disadvantage: true, tl: 3.into() }
    }
}

impl Config {
    /// Get typical TL7 default.
    pub fn default_tl7() -> Self {
        Self { female_as_5pts_disadvantage: false, tl: 7.into(), }
    }

    /// Get typical TL8 default.
    pub fn default_tl8() -> Self {
        Self { female_as_5pts_disadvantage: false, tl: 8.into(), }
    }
}
