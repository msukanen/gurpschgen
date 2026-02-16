#[cfg(test)]
/// Common things between a lots of tests around places.
pub mod common_between_tests {
    use crate::dta::locate_dta::locate_dta;

    /// `dta2json --auto --test` generated manifest file.
    pub(crate) const GCH_MANIFEST: &'static str = "test-gch.manifest";
    /// "Yrth" — a staple TL3 **GURPS Fantasy** genre.
    pub(crate) const TEST_GENRE_NAME_TL3: &'static str = "Yrth";
    /// "Space" — a staple TL10 **GURPS Space** genre.
    pub(crate) const TEST_GENRE_NAME_TL10: &'static str = "Space";

    /// Prepare test environment.
    pub(crate) fn prepare_test_environment() {
        let _ = env_logger::try_init();
        locate_dta(false);
    }
}
