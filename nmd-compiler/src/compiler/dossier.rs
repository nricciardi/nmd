mod document;

use std::sync::Arc;

pub use document::{Document, DocumentError};
use thiserror::Error;

use crate::compiler::{parsable::Parsable, compilable::Compilable};


use super::{compilable::{compilation_configuration::CompilationConfiguration, CompilationError}, loadable::{Loadable, LoadError}, parsable::{ParsingConfiguration, ParsingError}};

#[derive(Error, Debug)]
pub enum DossierError {
    #[error("dossier loading failed: '{0}'")]
    Load(&'static str)
}

pub struct Dossier {
    name: String,
    documents: Option<Vec<Document>>
}

/* impl Locatable for Dossier {
    fn location(self: &Self) -> &Location {
        &self.location
    }
} */

impl Loadable for Dossier {
    fn load(resource: super::resource::Resource) -> Result<Box<Self>, LoadError> {
        todo!()
    }
}

impl Parsable for Dossier {
    fn parse(&mut self, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError> {
        todo!()
    }
}

impl Compilable for Dossier {
    fn compile(&self, compilation_configuration: &CompilationConfiguration) -> Result<(), CompilationError> {
        todo!()
    }
}

impl Dossier {

    pub fn name(&self) -> &String {
        &self.name
    }

}
