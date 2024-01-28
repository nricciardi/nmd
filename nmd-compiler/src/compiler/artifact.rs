pub mod artifact_assets;

use std::path::PathBuf;

use thiserror::Error;

use self::artifact_assets::ArtifactAssets;

use super::{dumpable::{Dumpable, DumpError}, resource::{ResourceError, cached_disk_resource::CachedDiskResource}};


#[derive(Error, Debug)]
pub enum ArtifactError {
    #[error(transparent)]
    ResourceError(#[from] ResourceError)
}

pub struct Artifact {
    assets: Option<ArtifactAssets>,
    documents: Vec<CachedDiskResource>,
    root_location: PathBuf      // TODO: maybe remove... use relative path from .
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

    pub fn documents(&self) -> &Vec<CachedDiskResource> {
        &self.documents
    }

    fn set_documents(&mut self, documents: Vec<CachedDiskResource>) -> () {
        self.documents = documents
    }

    pub fn add_document(&mut self, document_name: &String, document_content: &String) -> Result<(), ArtifactError> {

        let final_location = self.root_location.join(document_name);

        let mut document = CachedDiskResource::try_from(final_location)?;

        document.set_cached_content(document_content);

        self.documents.push(document);

        Ok(())
    }
}

impl Dumpable<PathBuf> for Artifact {
    fn dump(output_path: PathBuf) -> Result<(), DumpError> {
        todo!()
    }
}