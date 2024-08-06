use gurpschgen_lib::skill::{DifficultyRating, SkillRoot, Stat};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{difficultyrating::difficulty_rating_from_match, stat::{stat_from_match, SkillLineage}};

fn skillroot_from_difficulty_rating(value: DifficultyRating) -> SkillRoot {
    SkillRoot::MA { diff: value }
}

fn skillroot_from_stat_and_difficulty_rating(value: (Stat, DifficultyRating)) -> SkillRoot {
    match value.0 {
        Stat::IQ => SkillRoot::M { stat: value.0, diff: value.1 },
        _        => SkillRoot::P { stat: value.0, diff: value.1 },
    }
}

pub(crate) fn skillroot_from_str(value: &str) -> SkillRoot {
    static RX_SKILL_BASE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?:\s*(?<base>MA?|P)\/(?<diff>E|A|V?H|S)(?:\s*\((?<stat>DX|HT|IQ|ST)\))?)").unwrap());
    if let Some(caps) = RX_SKILL_BASE.captures(value) {
        let base = caps.name("base").unwrap().as_str();
        let stat = caps.name("stat");
        match base {
            "M" => skillroot_from_stat_and_difficulty_rating((stat_from_match((SkillLineage::M, stat)), difficulty_rating_from_match(caps.name("diff")))),
            "MA" => skillroot_from_difficulty_rating(difficulty_rating_from_match(caps.name("diff"))),
            "P" => skillroot_from_stat_and_difficulty_rating((stat_from_match((SkillLineage::P, stat)), difficulty_rating_from_match(caps.name("diff")))),
            n => todo!("FATAL: base \"{n}\" not recognized!")
        }
    } else {
        todo!("FATAL: skill base \"{value}\" does not match specs (MA?|P)/(E|A|V?H)!")
    }
}
