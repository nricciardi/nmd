pub mod generator_configuration;


use std::{collections::HashMap, fs, path::PathBuf};
use nmd_core::{codex::modifier::constants::NEW_LINE, constants::DOSSIER_CONFIGURATION_YAML_FILE_NAME, dossier::{self, dossier_configuration::DossierConfiguration}, resource::{disk_resource::DiskResource, Resource, ResourceError}, utility::file_utility::{self, read_file_content}};
use once_cell::sync::Lazy;
use regex::Regex;
use crate::dossier_manager::{dossier_manager_configuration::DossierManagerConfiguration, DossierManager};

use self::generator_configuration::GeneratorConfiguration;


pub const WELCOME_FILE_NAME: &str = "welcome.nmd";

static SEARCH_HEADING_1_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?:^#[^#][ ]*(.+))").unwrap());


pub struct Generator {
}

impl Generator {

    /// Generate a new dossier based on GeneratorConfiguration
    pub fn generate_dossier(configuration: GeneratorConfiguration) -> Result<DossierConfiguration, ResourceError> {
        
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

                log::warn!("consider to use force flag");

                return Err(ResourceError::ResourceNotFound(format!("{}", configuration.path().to_string_lossy())))
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

        let mut dossier_configuration: DossierConfiguration;
        
        if !configuration.evaluate_existing_files() {
            
            dossier_configuration = DossierConfiguration::default();

        } else {

            dossier_configuration = DossierConfiguration::default().with_files_in_dir(configuration.path())?;
        }

        if let Some(name) = configuration.name() {
            dossier_configuration.set_name(name.clone());
        }

        if configuration.welcome() {

            let mut welcome_document = DiskResource::try_from(configuration.path().join(WELCOME_FILE_NAME))?;

            welcome_document.write("Welcome in **NMD**!")?;

            log::info!("added welcome page");

            let mut path = "./".to_string();
            path.push_str(WELCOME_FILE_NAME);
            
            dossier_configuration.set_raw_documents_paths(vec![path]);
        }

        dossier_configuration.dump_as_yaml(configuration.path().join(DOSSIER_CONFIGURATION_YAML_FILE_NAME))?;

        log::info!("added dossier configuration file: '{}'", DOSSIER_CONFIGURATION_YAML_FILE_NAME);

        Ok(dossier_configuration)
    }

    pub fn generate_dossier_from_markdown_file(markdown_source_file_path: &PathBuf, configuration: GeneratorConfiguration) -> Result<DossierConfiguration, ResourceError> {
        let markdown_file_content = read_file_content(markdown_source_file_path)?;

        let dossier_path = configuration.path().clone();
        let dossier_configuration = Self::generate_dossier(configuration)?;

        let dossier_manager = DossierManager::new(DossierManagerConfiguration::new(dossier_path));

        let mut current_nmd_file_content = String::new();
        let mut current_nmd_file_name: Option<String> = None;

        let mut document_names: HashMap<String, u32> = HashMap::new();

        let mut in_code_block = false;

        for line in markdown_file_content.lines() {

            if line.starts_with("```") {
                in_code_block = !in_code_block;
            }

            if !in_code_block && SEARCH_HEADING_1_REGEX.is_match(line) {

                log::info!("new header 1 found, generating new document...");

                if let Some(ref file_name) = current_nmd_file_name {

                    let mut file_name = String::from(file_name);

                    let n = *document_names.get(&file_name).unwrap_or(&0) + 1;
                    document_names.insert(file_name.clone(), n);

                    if let Some(n) = document_names.get(&file_name) {

                        let n = *n;

                        if n > 1 {
                            file_name = format!("{}-{}", file_name, n);
                        }

                    } else {
                        unreachable!();
                    }

                    dossier_manager.add_document(&file_name, &current_nmd_file_content).unwrap();
                    current_nmd_file_content.clear();
                }

                current_nmd_file_name = Some(String::from(SEARCH_HEADING_1_REGEX.captures(line).unwrap().get(1).unwrap().as_str()));

            }

            current_nmd_file_content.push_str(&format!("{}{}", line, NEW_LINE));
            
        }
        

        Ok(dossier_configuration)
    }
}

