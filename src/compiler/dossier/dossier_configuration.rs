mod dossier_configuration_style;
mod dossier_configuration_compilation;
mod dossier_configuration_path_reference;
mod dossier_configuration_path_reference_manager;
mod dossier_configuration_table_of_contents;

use std::collections::HashMap;
use std::io;
use std::path::PathBuf;

use dossier_configuration_table_of_contents::DossierConfigurationTableOfContents;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use log;

use crate::constants::{DOSSIER_CONFIGURATION_JSON_FILE_NAME, DOSSIER_CONFIGURATION_YAML_FILE_NAME, NMD_EXTENSION};
use crate::resource::text_reference::TextReferenceMap;
use crate::resource::Resource;
use crate::resource::{disk_resource::DiskResource, ResourceError};
use crate::utility::file_utility;

use self::dossier_configuration_path_reference::{DossierConfigurationPathReference, DossierConfigurationRawPathReference};
use self::dossier_configuration_path_reference_manager::DOSSIER_CONFIGURATION_RAW_REFERENCE_MANAGER;
use self::{dossier_configuration_compilation::DossierConfigurationCompilation, dossier_configuration_style::DossierConfigurationStyle};



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DossierConfiguration {
    #[serde(default = "default_name")]
    name: String,

    #[serde(rename(serialize = "toc", deserialize = "toc"), default = "default_toc")]
    table_of_contents_configuration: DossierConfigurationTableOfContents,

    #[serde(rename = "documents")]
    raw_documents_paths: Vec<DossierConfigurationRawPathReference>,

    #[serde(default = "default_style")]
    style: DossierConfigurationStyle,

    #[serde(default = "default_references")]
    references: TextReferenceMap,

    #[serde(default = "default_compilation")]
    compilation: DossierConfigurationCompilation,
}

fn default_name() -> String {
    "new-dossier".to_string()
}

fn default_style() -> DossierConfigurationStyle {
    DossierConfigurationStyle::default()
}

fn default_references() -> TextReferenceMap {
    HashMap::new()
}

fn default_compilation() -> DossierConfigurationCompilation {
    DossierConfigurationCompilation::default()
}

fn default_toc() -> DossierConfigurationTableOfContents {
    DossierConfigurationTableOfContents::default()
}


#[allow(dead_code)]
impl DossierConfiguration {

    pub fn new(root_path: PathBuf, name: String, toc: DossierConfigurationTableOfContents, raw_documents_paths: Vec<String>, style: DossierConfigurationStyle, reference: TextReferenceMap, compilation: DossierConfigurationCompilation) -> Self {
        
        DOSSIER_CONFIGURATION_RAW_REFERENCE_MANAGER.lock().unwrap().set_root_path(root_path);
        
        Self {
            name,
            table_of_contents_configuration: toc,
            raw_documents_paths,
            style,
            references: reference,
            compilation
        }
    }

    pub fn raw_documents_paths(&self) -> &Vec<String> {
        &self.raw_documents_paths
    }

    pub fn documents_paths(&self) -> Vec<DossierConfigurationPathReference> {

        let dcrfm = DOSSIER_CONFIGURATION_RAW_REFERENCE_MANAGER.lock().unwrap();

        if self.compilation.parallelization() {

            return self.raw_documents_paths.par_iter().map(|raw_reference| {
                dcrfm.parse_raw_reference(raw_reference, None)
            }).collect()

        } else {

            return self.raw_documents_paths.iter().map(|raw_reference| {
                dcrfm.parse_raw_reference(raw_reference, None)
            }).collect()
        }
    }

    pub fn set_raw_documents_paths(&mut self, documents: Vec<String>) -> () {
        self.raw_documents_paths = documents
    }

    pub fn append_raw_document_path(&mut self, raw_document_path: String) -> () {
        self.raw_documents_paths.push(raw_document_path)
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

    pub fn references(&self) -> &TextReferenceMap {
        &self.references
    }

    pub fn table_of_contents_configuration(&self) -> &DossierConfigurationTableOfContents {
        &self.table_of_contents_configuration
    }

    pub fn set_root_path(&mut self, root_path: PathBuf) {
        DOSSIER_CONFIGURATION_RAW_REFERENCE_MANAGER.lock().unwrap().set_root_path(root_path);
    }

    pub fn dump_as_yaml(&self, complete_output_path: PathBuf) -> Result<(), ResourceError> {
        let yaml_string = serde_yaml::to_string(&self).unwrap();

        let mut disk_resource = DiskResource::try_from(complete_output_path)?;

        disk_resource.erase()?;

        log::debug!("dump dossier configuration:\n{:#?}\n", self);
        disk_resource.write(&yaml_string)?;

        Ok(())
    }

    pub fn with_files_in_dir(mut self, dir_path: &PathBuf) -> Result<Self, io::Error> {
        let files = file_utility::all_files_in_dir(dir_path, &vec![NMD_EXTENSION.to_string()])?;

        self.raw_documents_paths = files.iter().map(|f| f.to_string_lossy().to_string()).collect();

        Ok(self)
    }
}

impl Default for DossierConfiguration {
    fn default() -> Self {
        Self {
            name: String::from("New Dossier"),
            raw_documents_paths: vec![],
            style: DossierConfigurationStyle::default(),
            references: HashMap::new(),
            compilation: DossierConfigurationCompilation::default(),
            table_of_contents_configuration: DossierConfigurationTableOfContents::default(),
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

                if file_name.to_string_lossy().eq(DOSSIER_CONFIGURATION_YAML_FILE_NAME) {

                    log::info!("{} found", DOSSIER_CONFIGURATION_YAML_FILE_NAME);

                    let mut config = Self::try_from_as_yaml(file_content)?;

                    config.set_root_path(path_buf.clone());

                    return Ok(config)
                }

                if file_name.to_string_lossy().eq(DOSSIER_CONFIGURATION_JSON_FILE_NAME) {

                    log::info!("{} found", DOSSIER_CONFIGURATION_JSON_FILE_NAME);

                    let mut config = Self::try_from_as_json(file_content)?;

                    config.set_root_path(path_buf.clone());

                    return Ok(config)
                }
            }
        }

        if path_buf.is_dir() {

            let yaml_path_buf = path_buf.join(DOSSIER_CONFIGURATION_YAML_FILE_NAME);

            if yaml_path_buf.exists() {

                let file_content = file_utility::read_file_content(&yaml_path_buf)?;

                let mut config = Self::try_from_as_yaml(file_content)?;

                config.set_root_path(path_buf.clone());

                return Ok(config)
            }

            let json_path_buf = path_buf.join(DOSSIER_CONFIGURATION_JSON_FILE_NAME);

            if json_path_buf.exists() {
                
                let file_content = file_utility::read_file_content(&json_path_buf)?;
                
                let mut config = Self::try_from_as_json(file_content)?;

                config.set_root_path(path_buf.clone());

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