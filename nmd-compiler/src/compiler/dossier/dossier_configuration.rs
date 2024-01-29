use std::{path::PathBuf, str::FromStr};

use thiserror::Error;


#[derive(Debug, Clone)]
pub enum DossierMetadata {

}

#[derive(Debug, Clone)]
pub struct DossierConfiguration {
    name: String,
    documents: Vec<String>,
    styles: Vec<String>,
    metadata: Vec<DossierMetadata>,
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

#[derive(Debug, Error)]
pub enum DossierConfigurationLoadError {

}

impl TryFrom<&PathBuf> for DossierConfiguration {
    type Error = DossierConfigurationLoadError;

    fn try_from(path_buf: &PathBuf) -> Result<Self, Self::Error> {

        // TODO: check if path_buf is a dir -> search nmd.json, or check if is path_buf is nmd.json

        todo!()
    }
}