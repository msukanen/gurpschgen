use crate::edition::GurpsEd;

pub struct Config {
    pub edition: GurpsEd,
    pub female_as_disadvantage: bool,
}

impl Config {
    pub fn default_tl3(edition: GurpsEd) -> Config {
        Config {
            edition,
            female_as_disadvantage: true,
        }
    }
}
