use std::path::PathBuf;

use getset::{Getters, Setters};

#[derive(Debug, Clone, Getters, Setters)]
pub struct DossierManagerConfiguration {

    #[getset(get = "pub", set = "pub")]
    dossier_path: PathBuf
}

impl DossierManagerConfiguration {
    pub fn new(dossier_path: PathBuf) -> Self {
        Self {
            dossier_path
        }
    }
}

impl Default for DossierManagerConfiguration {
    fn default() -> Self {
        Self {
            dossier_path: PathBuf::from(".")
        }
    }
}