pub mod chapter;

pub use chapter::Chapter;
use thiserror::Error;
use log;
use rayon::prelude::*;

use crate::compiler::parsable::{codex::parsing_rule::{parsing_configuration::{ParsingConfiguration, ParallelizationLevel}, parsing_result::{ParsingError, ParsingOutcome}}, Parsable};
use crate::compiler::{compilable::{Compilable, compilation_configuration::CompilationConfiguration, CompilationError}, resource::{Resource, ResourceError}};

#[derive(Error, Debug)]
pub enum DocumentError {
    #[error(transparent)]
    Load(#[from] ResourceError),

    #[error(transparent)]
    Parsing(#[from] ParsingError)
}

pub struct Document {
    name: String,
    chapters: Option<Vec<Chapter>>
}

impl TryFrom<Resource> for Document {
    type Error = DocumentError;

    fn try_from(resource: Resource) -> Result<Self, Self::Error> {
        Self::load(resource)
    }
}

impl Parsable for Document {

    fn parse(&mut self, parsing_configuration: &ParsingConfiguration) -> Result<(), ParsingError> {

        if let Some(mut chapters) = self.chapters {
            log::info!("parsing {} of document: '{}' (parallelization level: {:?})", chapters.len(), self.name, parsing_configuration.parallelization_level());

            if *parsing_configuration.parallelization_level() >= ParallelizationLevel::Medium {

                chapters.par_iter_mut().find_any(|chapter| {
                    chapter.parse(parsing_configuration).is_err()
                });

            } else {

                for chapter in chapters.iter() {
                    let _ = chapter.parse(parsing_configuration)?;
                }
            }

        } else {
            log::warn!("{} has not chapters", self.name)
        }

        Ok(())
    }
}

/* impl Compilable for Document {      // TODO: maybe remove
    fn compile(&self, compilation_configuration: &CompilationConfiguration) -> Result<(), CompilationError> {
        todo!()
    }
} */

impl Document {

    pub fn new(name: String, content: String) -> Self {
        
        let chapters = Option::None;

        if !content.is_empty() {
            chapters = Option::Some(content) 
        }

        Self {
            name,
            chapters
        }
    }

    pub fn chapters(&self) -> &Option<Vec<Chapter>> {
        &self.chapters
    }

    pub fn load(resource: Resource) -> Result<Self, DocumentError> {
        let mut content = resource.content()?;

        Ok(Self::new(resource.name().clone(), content))
    }
}