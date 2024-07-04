use std::{io, path::PathBuf};

use thiserror::Error;

use crate::{compiler::{dossier::{dossier_configuration::{self, DossierConfiguration}, Dossier}, loader::Loader}, constants::{DOSSIER_CONFIGURATION_YAML_FILE_NAME, NMD_EXTENSION}, resource::ResourceError, utility::file_utility};

use self::dossier_manager_configuration::DossierManagerConfiguration;

pub mod dossier_manager_configuration;

#[derive(Error, Debug)]
pub enum DossierManagerError {
    #[error(transparent)]
    ResourceError(#[from] ResourceError),

    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    SerdeYamlError(#[from] serde_yaml::Error)

}

/// Dossier manager, it can be used to add documents to a dossier
#[derive(Debug)]
pub struct DossierManager {
    configuration: DossierManagerConfiguration
}

impl DossierManager {
    pub fn new(configuration: DossierManagerConfiguration) -> Self {

        log::debug!("new DossierManager using configuration: \n{:#?}", configuration);

        Self {
            configuration
        }
    }

    /// Add document to dossier. Create file if it doesn't exist.
    pub fn add_document(&self, filename: &str, content: &str) -> Result<(), DossierManagerError> {

        let mut filename = String::from(filename);

        while filename.ends_with(&format!(".{}", NMD_EXTENSION)) {
            filename.remove(filename.len() - 1);
        }

        let filename = file_utility::build_output_file_name(&filename, NMD_EXTENSION);

        let mut dossier_configuration = DossierConfiguration::try_from(self.configuration.dossier_path())?;

        let abs_file_path = self.configuration.dossier_path().clone().join(&filename);
        let rel_file_path = format!(r"./{}", filename);

        file_utility::create_file_with_content(&abs_file_path, content)?;

        log::info!("created document: '{}'", filename);

        dossier_configuration.append_raw_document_path(rel_file_path);

        dossier_configuration.dump_as_yaml(self.configuration.dossier_path().clone().join(DOSSIER_CONFIGURATION_YAML_FILE_NAME))?;

        Ok(())
    }

    pub fn add_empty_document(&self, filename: &String) -> Result<(), DossierManagerError> {
        self.add_document(filename, "")
    }

    pub fn reset_dossier_configuration(&self, dossier_path: PathBuf, preserve_documents_list: bool) -> Result<(), DossierManagerError> {

        log::info!("resetting dossier configuration...");

        let mut dc: DossierConfiguration = DossierConfiguration::default();

        if preserve_documents_list {

            let ex_dc = DossierConfiguration::try_from(&dossier_path)?;

            dc.set_raw_documents_paths(ex_dc.raw_documents_paths().clone());
            log::info!("documents list will be preserved")
        }

        file_utility::create_file_with_content(
            &dossier_path.join(DOSSIER_CONFIGURATION_YAML_FILE_NAME),
            &serde_yaml::to_string(&dc)?
        )?;

        log::info!("reset done");

        Ok(())
    }
}