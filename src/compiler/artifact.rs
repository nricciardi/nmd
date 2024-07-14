pub mod artifact_assets;
pub mod artifacts_collection;

use std::fmt::Display;

use getset::{CopyGetters, Getters, MutGetters, Setters};
use thiserror::Error;


use crate::resource::{disk_resource::DiskResource, Resource, ResourceError};

use super::dumpable::{DumpConfiguration, DumpError, Dumpable};


#[derive(Error, Debug)]
pub enum ArtifactError {

    #[error("the output path must be an existing directory because artifact can contain more than one file")]
    OutputPathNotDir,

    #[error(transparent)]
    ResourceError(#[from] ResourceError)
}

pub type ArtifactContent = String;

#[derive(Debug, Clone, Getters, MutGetters, CopyGetters, Setters)]
pub struct Artifact {

    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    content: ArtifactContent,
}

impl Artifact {
    pub fn new(content: ArtifactContent) -> Self {

        Self {
            content
        }
    }
}

impl Display for Artifact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.content)
    }
}

impl Dumpable for Artifact {
    fn dump(&mut self, configuration: &DumpConfiguration) -> Result<(), DumpError> {

        let path = configuration.output_path().clone();

        log::info!("dump artifact in {:?}", path);

        let mut disk_resource = DiskResource::try_from(path)?;

        if configuration.force_dump() {
            disk_resource.create_parents_dir()?;
        }

        disk_resource.write(&self.content)?;

        Ok(())
    }
}

impl Into<String> for Artifact {
    fn into(self) -> String {
        self.content
    }
}