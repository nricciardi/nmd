pub mod chapter;

pub use chapter::Chapter;
use thiserror::Error;

use crate::compiler::{location::{Locatable, Location}, parsable::Parsable, compilable::{Compilable, compilable_configuration::CompilationConfiguration, CompilationError}, resource::Resource};

#[derive(Error, Debug)]
pub enum DocumentError {
    #[error("dossier loading failed: '{0}'")]
    Load(&'static str)
}

pub struct Document {
    location: Location,                 // TODO: maybe remove
    chapters: Option<Vec<Chapter>>
}

impl TryFrom<Resource> for Document {
    type Error = DocumentError;

    fn try_from(resource: Resource) -> Result<Self, Self::Error> {
        let mut content = resource.content();

        todo!()
    }
}


impl Locatable for Document {       // TODO: maybe remove
    fn location(&self) -> &Location {
        &self.location
    }
}

impl Parsable for Document {
    fn parse(&self, parsing_configuration: &chapter::ParsingConfiguration) -> chapter::ParsingResult {
        todo!()
    }
}

impl Compilable for Document {      // TODO: maybe remove
    fn compile(&self, compilation_configuration: &CompilationConfiguration) -> Result<(), CompilationError> {
        todo!()
    }
}

impl Document {

    pub fn new(content: String) {
        todo!()
    }

    pub fn chapters() -> Option<Vec<Chapter>> {
        todo!()
    }
}