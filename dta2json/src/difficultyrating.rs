use gurpschgen_lib::skill::DifficultyRating;
use regex::Match;

pub(crate) fn difficulty_rating_from_match(value: Option<Match<'_>>) -> DifficultyRating {
    match value {
        None => panic!("FATAL: ?!"),
        Some(m) => match m.as_str() {
            "E" => DifficultyRating::E,
            "A" => DifficultyRating::A,
            "H" => DifficultyRating::H,
            "S" => DifficultyRating::S,
            "VH" => DifficultyRating::VH,
            n => panic!("FATAL: unknown skill difficulty \"{n}\"!")
        }
    }
}
