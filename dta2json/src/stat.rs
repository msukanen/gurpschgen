use gurpschgen_lib::attrib::AttributeType;
use regex::Match;

/// Root base.
pub(crate) enum SkillLineage {
    /// Mental.
    M,
    /// Physical.
    P
}

pub(crate) fn stat_from_match(value: (SkillLineage, Option<Match<'_>>)) -> AttributeType {
    match value.1 {
        None => match value.0 {
            SkillLineage::P => AttributeType::DX,
            SkillLineage::M => AttributeType::IQ
        },
        Some(m) => stat_from_str(m.as_str())
    }
}

pub(crate) fn stat_from_str(value: &str) -> AttributeType {
    match value {
        "DX" => AttributeType::DX,
        "HT" => AttributeType::HT,
        "IQ" => AttributeType::IQ,
        "ST" => AttributeType::ST,
        n => todo!("FATAL: base stat \"{n}\" not recognized!")
    }
}
