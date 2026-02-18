#[cfg(test)]
/// Common things between a lots of tests around places.
pub mod common_between_tests {
    use std::{path::PathBuf, str::FromStr};

    use crate::dta::{genre::GenreManifestPackage, locate_dta::locate_dta};

    /// `dta2json --auto --test` generated manifest file.
    pub(crate) const GCH_MANIFEST: &'static str = "test-gch.manifest";
    /// "Yrth" — a staple TL3 **GURPS Fantasy** genre.
    pub(crate) const TEST_GENRE_NAME_TL3: &'static str = "Yrth";
    /// "Space" — a staple TL10 **GURPS Space** genre.
    pub(crate) const TEST_GENRE_NAME_TL10: &'static str = "Space";

    /// Prepare test environment.
    #[must_use = "Genre manifest is to be used!"]
    pub(crate) fn prepare_test_environment() -> GenreManifestPackage {
        let _ = env_logger::try_init();
        locate_dta(false);
        GenreManifestPackage::try_from(&PathBuf::from_str(GCH_MANIFEST).unwrap())
            .expect("Panix!")
    }
}
