pub mod artifact_assets;

use std::path::PathBuf;

use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};
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
    output_path: PathBuf
}

impl Artifact {

    pub fn new(output_path: PathBuf) -> Self {
        Self {
            assets: Option::None,
            documents: Vec::new(),
            output_path
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

        let final_location = self.output_path.join(document_name);

        let mut document = CachedDiskResource::try_from(final_location)?;

        document.set_cached_content(document_content);

        self.documents.push(document);

        Ok(())
    }
}

impl Dumpable for Artifact {
    fn dump(&self) -> Result<(), DumpError> {
        Ok(self.documents.par_iter_mut().for_each(|document| {
            document.dump_cached_content().unwrap()         // TODO: handle errors
        }))
    }
}