pub mod chapter;

pub use chapter::Chapter;
use thiserror::Error;

use crate::compiler::{location::{Locatable, Location}, parsable::Parsable, compilable::{Compilable, compilable_configuration::CompilationConfiguration, CompilationError}};

#[derive(Error, Debug)]
pub enum DocumentError {
    #[error("dossier loading failed: '{0}'")]
    Load(&'static str)
}

pub struct Document {
    location: Location,
    chapters: Option<Vec<Chapter> >
}

impl Locatable for Document {
    fn location(self: &Self) -> &Location {
        &self.location
    }
}

impl Parsable for Document {
    fn parse(&self, parsing_configuration: &chapter::ParsingConfiguration) -> chapter::ParsingResult {
        todo!()
    }
}

impl Compilable for Document {
    fn compile(&self, compilation_configuration: &CompilationConfiguration) -> Result<(), CompilationError> {
        todo!()
    }
}

impl Document {
    pub fn load(location: &Location) -> Result<Self, DocumentError> {
        todo!()
    }
}