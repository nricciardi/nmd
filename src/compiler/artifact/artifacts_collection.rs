use std::path::PathBuf;

use getset::{Getters, Setters};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::{compiler::dumpable::{DumpError, Dumpable}, resource::{cached_disk_resource::CachedDiskResource, Resource}};

use super::{artifact_assets::ArtifactAssets, Artifact, ArtifactError};


/// Final compilation result
#[derive(Debug, Getters, Setters)]
pub struct ArtifactsCollection {

    #[getset(get = "pub", set = "pub")]
    assets: Option<ArtifactAssets>,

    #[getset(get = "pub", set = "pub")]
    artifacts: Vec<Artifact>,

    #[getset(get = "pub", set = "pub")]
    output_path: PathBuf
}


#[allow(dead_code)]
impl ArtifactsCollection {

    pub fn new(output_path: PathBuf) -> Result<Self, ArtifactError> {

        if !output_path.is_dir() {
            return Err(ArtifactError::OutputPathNotDir)
        }

        Ok(Self {
            assets: Option::None,
            artifacts: Vec::new(),
            output_path
        })
    }

    // TODO
    // pub fn add_artifact(&mut self, document_name: &String, document_content: &String) -> Result<(), ArtifactError> {

    //     let final_location = self.output_path.join(document_name);

    //     let mut document = CachedDiskResource::try_from(final_location)?;

    //     document.set_cached_content(document_content);

    //     self.artifacts.push(document);

    //     Ok(())
    // }
}

impl Dumpable for ArtifactsCollection {
    fn dump(&mut self) -> Result<(), DumpError> {

        log::info!("dump artifacts collection...",);

        let error = self.artifacts.par_iter_mut().map(|artifact| {

            log::info!("dumping artifact in {:?}", artifact.output_path());

            artifact.dump()
        })
        .find_any(|result| result.is_err());

        if let Some(error) = error {
            return Err(error.err().unwrap());
        }

        Ok(())
    }
}