use std::{io, path::PathBuf};

use clap::error;
use thiserror::Error;

use crate::{compiler::dossier::{dossier_configuration::{self, DossierConfiguration}, Dossier}, resource::ResourceError, utility::file_utility};

use self::dossier_manager_configuration::DossierManagerConfiguration;

pub mod dossier_manager_configuration;

#[derive(Error, Debug)]
pub enum DossierManagerError {
    #[error(transparent)]
    ResourceError(#[from] ResourceError),

    #[error(transparent)]
    IoError(#[from] io::Error),

}

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

    pub fn add_document(&self, filename: &String) -> Result<(), DossierManagerError> {

        let mut dossier_configuration = DossierConfiguration::try_from(self.configuration.dossier_path())?;

        let abs_file_path = self.configuration.dossier_path().clone().join(filename);
        let rel_file_path = format!(r"./{}", filename);

        file_utility::create_empty_file(&abs_file_path)?;

        log::info!("created document: '{}'", filename);

        dossier_configuration.append_raw_document_path(rel_file_path);

        dossier_configuration.dump_as_yaml(self.configuration.dossier_path().clone().join(dossier_configuration::YAML_FILE_NAME))?;

        Ok(())
    }
}