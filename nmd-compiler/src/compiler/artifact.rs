pub mod artifact_assets;

use std::path::PathBuf;

use self::artifact_assets::ArtifactAssets;

use super::{dumpable::Dumpable, resource::Resource};

pub struct Artifact {
    assets: ArtifactAssets,
    documents: Vec<Resource>
}

impl Artifact {
    pub fn assets(&self) -> &ArtifactAssets {
        &self.assets
    }

    pub fn documents(&self) -> &Vec<Resource> {
        &self.documents
    }

    pub fn set_documents(&mut self, documents: Vec<Resource>) -> () {
        self.documents = documents
    }

    pub fn append_documents(&mut self, documents: Resource) -> () {
        self.documents.push(documents)
    }
}

impl Dumpable for Artifact {
    
}