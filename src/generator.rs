pub mod generator_configuration;

use std::{fs, path::PathBuf};
use self::generator_configuration::GeneratorConfiguration;
use crate::{compiler::dossier::{self, dossier_configuration::{self, DossierConfiguration}}, resource::{disk_resource::DiskResource, Resource, ResourceError}, utility::file_utility};

pub const WELCOME_FILE_NAME: &str = "welcome.nmd";

pub struct Generator {
}

impl Generator {

    // TODO: move configuration in struct
    pub fn generate_dossier(configuration: GeneratorConfiguration) -> Result<(), ResourceError> {
        
        if configuration.path().exists() {

            if !configuration.path().is_dir() {
                return Err(ResourceError::InvalidResourceVerbose("not a directory".to_string()))
            }

            if let Ok(entries) = fs::read_dir(&configuration.path()) {
                if entries.count() != 0 && !configuration.force_generation() {
                    return Err(ResourceError::InvalidResourceVerbose("directory not empty, try to use force option".to_string()))
                }
            } else {
                return Err(ResourceError::ReadError("check permission".to_string()))
            }

            if configuration.force_generation() {
                fs::remove_dir_all(configuration.path().clone())?;
                log::info!("cleared {}", configuration.path().to_string_lossy());

                if let Err(err) = fs::create_dir_all(&configuration.path()) {
                    return Err(ResourceError::ReadError(err.to_string()))
                }

                log::info!("created dossier directory");
            }

        } else {
            if !configuration.force_generation() {
                return Err(ResourceError::ResourceNotFound(configuration.path().to_string_lossy().to_string()))
            }

            if let Err(err) = fs::create_dir_all(&configuration.path()) {
                return Err(ResourceError::ReadError(err.to_string()))
            }

            log::info!("created dossier directory");
        }

        let assets_path = configuration.path().join(dossier::ASSETS_DIR);

        file_utility::create_directory(&assets_path)?;
        log::info!("added {}/ directory", dossier::ASSETS_DIR);

        file_utility::create_directory(&assets_path.join(dossier::IMAGES_DIR))?;
        log::info!("added {}/{} directory", dossier::ASSETS_DIR, dossier::IMAGES_DIR);

        file_utility::create_directory(&assets_path.join(dossier::DOCUMENTS_DIR))?;
        log::info!("added {}/{} directory", dossier::ASSETS_DIR, dossier::DOCUMENTS_DIR);

        file_utility::create_directory(&assets_path.join(dossier::STYLES_DIR))?;
        log::info!("added {}/{} directory", dossier::ASSETS_DIR, dossier::STYLES_DIR);

        if configuration.gitkeep() {

            file_utility::create_empty_file(&assets_path.join("images").join(".gitkeep"))?;
            file_utility::create_empty_file(&assets_path.join("documents").join(".gitkeep"))?;
            file_utility::create_empty_file(&assets_path.join("styles").join(".gitkeep"))?;

            log::info!("added .gitkeep files");
        }

        let mut dossier_configuration = DossierConfiguration::default();

        if configuration.welcome() {

            let mut welcome_document = DiskResource::try_from(configuration.path().join(WELCOME_FILE_NAME))?;

            welcome_document.write("Welcome in **NMD**!")?;

            log::info!("added welcome page");

            let mut path = "./".to_string();
            path.push_str(WELCOME_FILE_NAME);
            
            dossier_configuration.set_raw_documents_paths(vec![path]);
        }

        dossier_configuration.dump_as_yaml(configuration.path().join(dossier_configuration::YAML_FILE_NAME))?;

        log::info!("added dossier configuration file: '{}'", dossier_configuration::YAML_FILE_NAME);

        Ok(())
    }

}

