use std::fmt::Display;

pub(crate) enum LegacyFileType {
    DTA,
    GEN,
}

impl Display for LegacyFileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::DTA => "dta",
            Self::GEN => "gen",
        })
    }
}
