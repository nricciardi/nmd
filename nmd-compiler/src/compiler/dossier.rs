mod document;

pub use document::{Document, DocumentError};
use thiserror::Error;

use crate::compiler::{location::{Location, Locatable}, parsable::Parsable, compilable::Compilable};

use super::compilable::{compilable_configuration::CompilationConfiguration, CompilationError};

#[derive(Error, Debug)]
pub enum DossierError {
    #[error("dossier loading failed: '{0}'")]
    Load(&'static str)
}

pub struct Dossier {
    location: Location,
    documents: Option<Vec<Document>>
}

impl Locatable for Dossier {
    fn location(self: &Self) -> &Location {
        &self.location
    }
}

impl Parsable for Dossier {
    fn parse(&self, parsing_configuration: &document::chapter::ParsingConfiguration) -> document::chapter::ParsingResult {
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
        self.location.resource_name()
    }

    pub fn load(location: &Location) -> Result<Self, DossierError> {

        todo!()

        /* Self {
            location,
            documents
        } */
    }
}
