use std::fmt::Display;

/// Legacy SJG MakeChar file extensions.
pub enum LegacyFileExt {
    DTA,
    GEN,
}

impl Display for LegacyFileExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::DTA => "dta",
            Self::GEN => "gen",
        })
    }
}

/// Our file types.
pub enum FileExt {
    Data,
    Genre,
    Configuration,
}

impl Display for FileExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Configuration => "config",
            Self::Data => "json",
            Self::Genre => "genre",
        })
    }
}
