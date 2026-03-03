//! Global "vault" for genre-specific stuff dwells here.

use std::{collections::HashMap, path::PathBuf, str::FromStr, sync::{Arc, RwLock}};

use once_cell::sync::Lazy;

use crate::{dta::genre::{GENRE_MANIFEST_FN, GenreManifestPackage}, id::RuntimeID, misc::category::CategoryPayload};

/// Genre specific vault of data.
pub struct Vault {
    pub source: GenreManifestPackage,
    pub registry: RwLock<HashMap<RuntimeID, CategoryPayload>>,
}

pub static VAULT: Lazy<Arc<Vault>> = Lazy::new(|| {
    Arc::new(Vault {
        source: GenreManifestPackage::try_from(&PathBuf::from_str(GENRE_MANIFEST_FN).unwrap())
            .unwrap_or_else(|e| panic!("{e:?}")),
        registry: RwLock::new(HashMap::new()),
    })
});
