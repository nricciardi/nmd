pub mod chapter;

pub use chapter::Chapter;
use thiserror::Error;
use log;

use crate::compiler::{parsable::{Parsable, ParsingConfiguration}, compilable::{Compilable, compilable_configuration::CompilationConfiguration, CompilationError}, resource::{Resource, ResourceError}};

#[derive(Error, Debug)]
pub enum DocumentError {
    #[error(transparent)]
    Load(#[from] ResourceError)
}

pub struct Document {
    chapters: Option<Vec<Chapter>>
}

impl TryFrom<Resource> for Document {
    type Error = DocumentError;

    fn try_from(resource: Resource) -> Result<Self, Self::Error> {
        Self::load(resource)
    }
}


impl Parsable for Document {
    fn parse(&self, parsing_configuration: &ParsingConfiguration) -> chapter::ParsingResult {
        match self.chapters {
            Some(chapters) => {
                
                todo!()         // log... log::info!("parsing")

                chapters.iter().for_each(|chapter| chapter.parse(parsing_configuration))
            }
            None => 
        }
    }
}

/* impl Compilable for Document {      // TODO: maybe remove
    fn compile(&self, compilation_configuration: &CompilationConfiguration) -> Result<(), CompilationError> {
        todo!()
    }
} */

impl Document {

    pub fn new(content: String) -> Self {
        
        if content.is_empty() {
            return Self {
                        chapters: Option::None
                    }
        }

        todo!()
    }

    pub fn chapters(&self) -> &Option<Vec<Chapter>> {
        &self.chapters
    }

    pub fn load(resource: Resource) -> Result<Self, DocumentError> {
        let mut content = resource.content()?;

        Ok(Self::new(content))
    }
}