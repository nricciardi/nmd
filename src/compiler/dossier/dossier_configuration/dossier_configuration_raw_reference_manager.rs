use std::path::PathBuf;

use super::dossier_configuration_raw_reference::DossierConfigurationRawReference;

pub struct DossierConfigurationRawReferenceManager {
    root_path: Option<PathBuf>,
    not_unix_like_os: bool
}

impl DossierConfigurationRawReferenceManager {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_root_path(&mut self, root_path: PathBuf) {
        self.root_path = Some(root_path)
    }

    pub fn manage_raw_reference(&self, raw_reference: &DossierConfigurationRawReference) -> DossierConfigurationRawReference {
        // TODO
    }
}

impl Default for DossierConfigurationRawReferenceManager {
    fn default() -> Self {
        Self {
            root_path: None,
            not_unix_like_os: false
        }
    }
}