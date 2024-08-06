use gurpschgen_lib::skill::Stat;
use regex::Match;

/// Root base.
pub(crate) enum SkillLineage {
    /// Mental.
    M,
    /// Physical.
    P
}

pub(crate) fn stat_from_match(value: (SkillLineage, Option<Match<'_>>)) -> Stat {
    match value.1 {
        None => match value.0 {
            SkillLineage::P => Stat::DX,
            SkillLineage::M => Stat::IQ
        },
        Some(m) => stat_from_str(m.as_str())
    }
}

pub(crate) fn stat_from_str(value: &str) -> Stat {
    match value {
        "DX" => Stat::DX,
        "HT" => Stat::HT,
        "IQ" => Stat::IQ,
        "ST" => Stat::ST,
        n => todo!("FATAL: base stat \"{n}\" not recognized!")
    }
}
