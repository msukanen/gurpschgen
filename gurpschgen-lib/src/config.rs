use crate::edition::GurpsEd;

pub struct Config {
    pub edition: GurpsEd,
    pub female_as_5pts_disadvantage: bool,
    pub tl: i32,
}

impl Config {
    pub fn default_tl3(edition: GurpsEd) -> Self {
        Self { edition, female_as_5pts_disadvantage: true, tl: 3, }
    }

    pub fn default_tl7(edition: GurpsEd) -> Self {
        Self { edition, female_as_5pts_disadvantage: false, tl: 7, }
    }

    pub fn default_tl8(edition: GurpsEd) -> Self {
        Self { edition, female_as_5pts_disadvantage: false, tl: 8, }
    }
}
