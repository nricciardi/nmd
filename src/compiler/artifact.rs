pub mod artifact_assets;
pub mod artifacts_collection;

use std::path::PathBuf;

use getset::{Getters, MutGetters, Setters};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use thiserror::Error;


use crate::resource::{cached_disk_resource::CachedDiskResource, Resource, ResourceError};

use self::artifact_assets::ArtifactAssets;

use super::dumpable::{Dumpable, DumpError};


#[derive(Error, Debug)]
pub enum ArtifactError {

    #[error("the output path must be an existing directory because artifact can contain more than one file")]
    OutputPathNotDir,

    #[error(transparent)]
    ResourceError(#[from] ResourceError)
}



#[derive(Debug, Clone, Getters, MutGetters, Setters)]
pub struct Artifact {

    #[getset(get = "pub", set = "pub")]
    output_path: PathBuf,

    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    content: CachedDiskResource,
}

impl Artifact {
    pub fn new(output_path: PathBuf) -> Result<Self, ArtifactError> {

        Ok(Self {
            content: CachedDiskResource::try_from(output_path.clone())?,
            output_path
        })
    }
}

impl Dumpable for Artifact {
    fn dump(&mut self) -> Result<(), DumpError> {

        log::info!("dump artifact...",);

        self.content.dump_cached_content();

        Ok(())
    }
}