use std::path::PathBuf;

#[derive(Debug)]
pub struct DossierManagerConfiguration {
    dossier_path: PathBuf
}

impl DossierManagerConfiguration {
    pub fn new(dossier_path: PathBuf) -> Self {
        Self {
            dossier_path
        }
    }

    pub fn dossier_path(&self) -> &PathBuf {
        &self.dossier_path
    }
}

impl Default for DossierManagerConfiguration {
    fn default() -> Self {
        Self {
            dossier_path: PathBuf::from(".")
        }
    }
}