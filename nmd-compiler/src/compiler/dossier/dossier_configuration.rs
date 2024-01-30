use std::{io, path::{PathBuf, MAIN_SEPARATOR, MAIN_SEPARATOR_STR}};

use serde::Deserialize;
use serde_json::error;
use thiserror::Error;
use log;

use crate::compiler::{resource::ResourceError, utility::file_utility};


#[derive(Debug, Clone, Deserialize)]
pub enum DossierMetadata {

}

#[derive(Debug, Clone, Deserialize)]
pub struct DossierConfiguration {
    #[serde(default = "default_name")]
    name: String,

    documents: Vec<String>,

    #[serde(default = "default_styles")]
    styles: Vec<String>,

    #[serde(default = "default_metadata")]
    metadata: Vec<DossierMetadata>,
}

fn default_name() -> String {
    "new-dossier".to_string()
}

fn default_styles() -> Vec<String> {
    vec![]      // TODO
}

fn default_metadata() -> Vec<DossierMetadata> {
    vec![]
}

impl DossierConfiguration {

    pub fn new(name: String, documents: Vec<String>, styles: Vec<String>, metadata: Vec<DossierMetadata>) -> Self {
        Self {
            name,
            documents,
            styles,
            metadata
        }
    }

    pub fn documents(&self) -> &Vec<String> {
        &self.documents
    }

    pub fn set_documents(&mut self, documents: Vec<String>) -> () {
        self.documents = documents
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn apply_root_path(&mut self, root_path: &PathBuf) -> () {

        let replacement_pattern: String = ".".to_string() + MAIN_SEPARATOR_STR;

        for document in self.documents.iter_mut() {

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
}

impl Default for DossierConfiguration {
    fn default() -> Self {
        Self {
            name: String::from("New Dossier"),
            documents: vec![],          // TODO: all .nmd file in running directory
            styles: vec![],              // TODO: default style
            metadata: vec![],
        }
    }
}


impl DossierConfiguration {
    fn try_from_as_yaml(content: String) -> Result<Self, ResourceError> {

        log::info!("try to load dossier configuration from yaml...");
        
        match serde_yaml::from_str(&content) {
            Ok(config) => return Ok(config),
            Err(e) => return Err(ResourceError::InvalidResourceVerbose(e.to_string()))
        }
    }

    fn try_from_as_json(content: String) -> Result<Self, ResourceError> {

        log::info!("try to load dossier configuration from json...");

        match serde_json::from_str(&content) {
            Ok(config) => return Ok(config),
            Err(e) => return Err(ResourceError::InvalidResourceVerbose(e.to_string()))
        }
    }
}

impl TryFrom<&PathBuf> for DossierConfiguration {
    type Error = ResourceError;

    fn try_from(path_buf: &PathBuf) -> Result<Self, Self::Error> {

        const YAML_FILE_NAME: &str = "nmd.yml";
        const JSON_FILE_NAME: &str = "nmd.json";

        if path_buf.is_file() {
            if let Some(file_name) = path_buf.file_name() {

                let file_content = file_utility::read_file_content(path_buf)?;

                if file_name.to_string_lossy().eq(YAML_FILE_NAME) {
                    let mut config = Self::try_from_as_yaml(file_content)?;

                    config.apply_root_path(path_buf);

                    return Ok(config)
                }

                if file_name.to_string_lossy().eq(JSON_FILE_NAME) {
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

        assert_eq!(configuration.documents()[0], nmd_dossier_path.join("d1.nmd").to_string_lossy().to_string())
    }
}