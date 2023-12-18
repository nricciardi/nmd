pub mod artifact_assets;

use std::path::PathBuf;

use thiserror::Error;

use self::artifact_assets::ArtifactAssets;

use super::{dumpable::Dumpable, resource::{Resource, ResourceError}};


#[derive(Error, Debug)]
pub enum ArtifactError {
    #[error(transparent)]
    ResourceError(#[from] ResourceError)
}

pub struct Artifact {
    assets: Option<ArtifactAssets>,
    documents: Vec<Resource>,
    root_location: PathBuf
}

impl Artifact {

    pub fn new(root_location: PathBuf) -> Self {
        Self {
            assets: Option::None,
            documents: Vec::new(),
            root_location
        }
    }

    pub fn assets(&self) -> &Option<ArtifactAssets> {
        &self.assets
    }

    pub fn documents(&self) -> &Vec<Resource> {
        &self.documents
    }

    fn set_documents(&mut self, documents: Vec<Resource>) -> () {
        self.documents = documents
    }

    pub fn add_document(&mut self, document_name: &String, document_content: &String) -> Result<(), ArtifactError> {

        let final_location = self.root_location.join(document_name);

        let document = Resource::new(final_location)?;

        document.write(document_content)?;

        self.documents.push(document);

        Ok(())
    }
}

impl Dumpable for Artifact {
    
}