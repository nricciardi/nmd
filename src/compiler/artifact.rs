pub mod artifact_assets;

use std::path::PathBuf;

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use thiserror::Error;


use crate::resource::{cached_disk_resource::CachedDiskResource, Resource, ResourceError};

use self::artifact_assets::ArtifactAssets;

use super::dumpable::{Dumpable, DumpError};


#[derive(Error, Debug)]
pub enum ArtifactError {

    #[error("the output path must be an existing directory")]
    OutputPathNotDir,

    #[error(transparent)]
    ResourceError(#[from] ResourceError)
}


/// Final compilation result
pub struct Artifact {
    assets: Option<ArtifactAssets>,
    documents: Vec<CachedDiskResource>,
    output_path: PathBuf
}


#[allow(dead_code)]
impl Artifact {

    pub fn new(output_path: PathBuf) -> Result<Self, ArtifactError> {

        if !output_path.is_dir() {
            return Err(ArtifactError::OutputPathNotDir)
        }

        Ok(Self {
            assets: Option::None,
            documents: Vec::new(),
            output_path
        })
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
    fn dump(&mut self) -> Result<(), DumpError> {

        log::info!("dump artifact...",);

        let error = self.documents.par_iter_mut().map(|document| {

            log::info!("dumping document in {:?}", document.location());

            document.dump_cached_content()
        })
        .find_any(|result| result.is_err());

        if let Some(error) = error {
            return Err(DumpError::ResourceError(error.err().unwrap()));
        }

        Ok(())
    }
}