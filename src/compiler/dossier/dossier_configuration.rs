mod dossier_configuration_style;
mod dossier_configuration_metadata;
mod dossier_configuration_compilation;

use std::path::{PathBuf, MAIN_SEPARATOR_STR};

use serde::{Deserialize, Serialize};
use log;

use crate::resource::Resource;
use crate::resource::{disk_resource::DiskResource, ResourceError};
use crate::utility::file_utility;

use self::{dossier_configuration_compilation::DossierConfigurationCompilation, dossier_configuration_metadata::DossierConfigurationMetadata, dossier_configuration_style::DossierConfigurationStyle};


pub const YAML_FILE_NAME: &str = "nmd.yml";
pub const JSON_FILE_NAME: &str = "nmd.json";


#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DossierConfiguration {
    #[serde(default = "default_name")]
    name: String,

    raw_documents_paths: Vec<String>,

    #[serde(default = "default_style")]
    style: DossierConfigurationStyle,

    #[serde(default = "default_metadata")]
    metadata: DossierConfigurationMetadata,

    #[serde(default = "default_compilation")]
    compilation: DossierConfigurationCompilation,
}

fn default_name() -> String {
    "new-dossier".to_string()
}

fn default_style() -> DossierConfigurationStyle {
    DossierConfigurationStyle::default()
}

fn default_metadata() -> DossierConfigurationMetadata {
    DossierConfigurationMetadata::default()
}

fn default_compilation() -> DossierConfigurationCompilation {
    DossierConfigurationCompilation::default()
}


#[allow(dead_code)]
impl DossierConfiguration {

    pub fn new(name: String, raw_documents_paths: Vec<String>, style: DossierConfigurationStyle, metadata: DossierConfigurationMetadata, compilation: DossierConfigurationCompilation) -> Self {
        Self {
            name,
            raw_documents_paths,
            style,
            metadata,
            compilation
        }
    }

    pub fn raw_documents_paths(&self) -> &Vec<String> {
        &self.raw_documents_paths
    }

    pub fn set_raw_documents_paths(&mut self, documents: Vec<String>) -> () {
        self.raw_documents_paths = documents
    }

    pub fn append_raw_document_path(&mut self, mut document_path: String) -> () {
        self.raw_documents_paths.push(document_path)
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn style(&self) -> &DossierConfigurationStyle {
        &self.style
    }

    pub fn compilation(&self) -> &DossierConfigurationCompilation {
        &self.compilation
    }

    pub fn apply_root_path(&mut self, root_path: &PathBuf) -> () {

        let replacement_pattern: String = ".".to_string() + MAIN_SEPARATOR_STR;

        for document in self.raw_documents_paths.iter_mut() {

            if document.starts_with(replacement_pattern.as_str()) {

                // remove . + MAIN_SEPARATOR_STR
                for _ in 0..replacement_pattern.len() {
                    document.remove(0);
                }

                let new_path = root_path.join(&document);

                document.clear();
                
                document.push_str(new_path.to_string_lossy().to_string().as_str());
            }
        }
    }

    pub fn dump_as_yaml(&self, complete_output_path: PathBuf) -> Result<(), ResourceError> {
        let yaml_string = serde_yaml::to_string(&self).unwrap();

        let mut disk_resource = DiskResource::try_from(complete_output_path)?;

        disk_resource.erase()?;

        log::debug!("dump dossier configuration:\n{:#?}\n", self);
        disk_resource.write(&yaml_string)?;

        Ok(())
    }
}

impl Default for DossierConfiguration {
    fn default() -> Self {
        Self {
            name: String::from("New Dossier"),
            raw_documents_paths: vec![],          // TODO: all .nmd file in running directory
            style: DossierConfigurationStyle::default(),
            metadata: DossierConfigurationMetadata::default(),
            compilation: DossierConfigurationCompilation::default()
        }
    }
}


impl DossierConfiguration {
    fn try_from_as_yaml(content: String) -> Result<Self, ResourceError> {

        log::info!("try to load dossier configuration from yaml content...");
        
        match serde_yaml::from_str(&content) {
            Ok(config) => {
                log::info!("dossier configuration loaded from yaml");
                return Ok(config)
            },
            Err(e) => return Err(ResourceError::InvalidResourceVerbose(e.to_string()))
        }
    }

    fn try_from_as_json(content: String) -> Result<Self, ResourceError> {

        log::info!("try to load dossier configuration from json content...");

        match serde_json::from_str(&content) {
            Ok(config) => {
                log::info!("dossier configuration loaded from json");
                return Ok(config)
            },
            Err(e) => return Err(ResourceError::InvalidResourceVerbose(e.to_string()))
        }
    }

    pub fn load(path_buf: &PathBuf) -> Result<Self, ResourceError> {
        Self::try_from(path_buf)
    }
}

impl TryFrom<&PathBuf> for DossierConfiguration {
    type Error = ResourceError;

    fn try_from(path_buf: &PathBuf) -> Result<Self, Self::Error> {

        if path_buf.is_file() {
            if let Some(file_name) = path_buf.file_name() {

                let file_content = file_utility::read_file_content(path_buf)?;

                if file_name.to_string_lossy().eq(YAML_FILE_NAME) {

                    log::info!("{} found", YAML_FILE_NAME);

                    let mut config = Self::try_from_as_yaml(file_content)?;

                    config.apply_root_path(path_buf);

                    return Ok(config)
                }

                if file_name.to_string_lossy().eq(JSON_FILE_NAME) {

                    log::info!("{} found", JSON_FILE_NAME);

                    let mut config = Self::try_from_as_json(file_content)?;

                    config.apply_root_path(path_buf);

                    return Ok(config)
                }
            }
        }

        if path_buf.is_dir() {

            let yaml_path_buf = path_buf.join(YAML_FILE_NAME);

            if yaml_path_buf.exists() {

                let file_content = file_utility::read_file_content(&yaml_path_buf)?;

                let mut config = Self::try_from_as_yaml(file_content)?;

                config.apply_root_path(path_buf);

                return Ok(config)
            }

            let json_path_buf = path_buf.join(JSON_FILE_NAME);

            if json_path_buf.exists() {
                
                let file_content = file_utility::read_file_content(&json_path_buf)?;
                
                let mut config = Self::try_from_as_json(file_content)?;

                config.apply_root_path(path_buf);

                return Ok(config)
            }
        }

        Err(ResourceError::ResourceNotFound("dossier configuration".to_string()))
    }
}


#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use super::*;


    #[test]
    fn apply_root_path() {
        let project_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let dossier_dir = "nmd-test-dossier-1";
        let nmd_dossier_path = project_directory.join("test-resources").join(dossier_dir);

        let configuration = DossierConfiguration::try_from(&nmd_dossier_path).unwrap();

        assert_eq!(configuration.raw_documents_paths()[0], nmd_dossier_path.join("d1.nmd").to_string_lossy().to_string())
    }
}