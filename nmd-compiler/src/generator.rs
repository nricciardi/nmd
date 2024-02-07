pub mod generator_configuration;

use std::{fs, path::PathBuf};
use self::generator_configuration::GeneratorConfiguration;
use crate::{compiler::dossier::dossier_configuration::{self, DossierConfiguration}, resource::{disk_resource::DiskResource, Resource, ResourceError}, utility::file_utility};

pub const WELCOME_FILE_NAME: &str = "welcome.nmd";

pub struct Generator {
}

impl Generator {
    pub fn generate_dossier(configuration: GeneratorConfiguration) -> Result<(), ResourceError> {
        
        if configuration.input_path().exists() {

            if !configuration.input_path().is_dir() {
                return Err(ResourceError::InvalidResourceVerbose("not a directory".to_string()))
            }

            if let Ok(entries) = fs::read_dir(&configuration.input_path()) {
                if entries.count() != 0 && !configuration.force_generation() {
                    return Err(ResourceError::InvalidResourceVerbose("directory not empty, try to use force option".to_string()))
                }
            } else {
                return Err(ResourceError::ReadError("check permission".to_string()))
            }

        } else {
            if !configuration.force_generation() {
                return Err(ResourceError::ResourceNotFound(configuration.input_path().to_string_lossy().to_string()))
            }

            if let Err(err) = fs::create_dir_all(&configuration.input_path()) {
                return Err(ResourceError::ReadError(err.to_string()))
            }
        }

        let assets_path = configuration.input_path().join("assets");

        if assets_path.exists() && configuration.force_generation() {
            fs::remove_dir_all(assets_path.clone())?;
        }


        log::info!("add assets/ directory");

        file_utility::create_directory(&assets_path)?;
        file_utility::create_directory(&assets_path.join("images"))?;
        file_utility::create_directory(&assets_path.join("documents"))?;
        file_utility::create_directory(&assets_path.join("styles"))?;

        if configuration.gitkeep() {
            log::info!("add .gitkeep files");

            file_utility::create_empty_file(&assets_path.join("images").join(".gitkeep"))?;
            file_utility::create_empty_file(&assets_path.join("documents").join(".gitkeep"))?;
            file_utility::create_empty_file(&assets_path.join("styles").join(".gitkeep"))?;
        }

        let mut dossier_configuration = DossierConfiguration::default();

        if configuration.welcome() {

            log::info!("add welcome...");

            let mut welcome_document = DiskResource::try_from(configuration.input_path().join(WELCOME_FILE_NAME))?;

            welcome_document.write("Welcome in NMD!")?;

            dossier_configuration.set_documents(vec![WELCOME_FILE_NAME.to_string()]);
        }

        let yaml_string = serde_yaml::to_string(&dossier_configuration).unwrap();

        let mut disk_resource = DiskResource::try_from(configuration.input_path().join(dossier_configuration::YAML_FILE_NAME))?;

        if configuration.force_generation() {
            disk_resource.erase()?;
        }       

        log::info!("add {}", dossier_configuration::YAML_FILE_NAME);
        disk_resource.write(&yaml_string)?;

        Ok(())
    }

}

