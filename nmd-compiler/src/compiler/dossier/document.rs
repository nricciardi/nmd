pub mod chapter;

use std::sync::{Arc, RwLock};

pub use chapter::Chapter;
use thiserror::Error;
use log;
use rayon::prelude::*;

use crate::compiler::parsable::{ParsingError, Parsable, ParallelParsable, SerialParsable};
use crate::compiler::parsable::parsing_configuration::{ParsingConfiguration, ParallelizationLevel};
use crate::compiler::loadable::{Loadable, LoadError};
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

impl Loadable for Document {
    fn load(resource: Resource) -> Result<Box<Self>, LoadError> {
        let mut content = resource.content()?;

        Ok(Box::new(Self::new(resource.name().clone(), content)))
    }
}

impl ParallelParsable for Document {

    fn parallel_parse(&mut self, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError> {

        if let Some(mut chapters) = std::mem::take(&mut self.chapters) {

            let pool = rayon::ThreadPoolBuilder::new().num_threads(chapters.len()).build().unwrap();

             let maybe_one_failed = chapters.par_iter_mut()
            .map(|chapter| {

                chapter.parse(Arc::clone(&parsing_configuration))
            
            }).find_any(|result| result.is_err());
    
            if let Some(Err(err)) = maybe_one_failed {
                return Err(err);
            }
        }

        Ok(())
    }
}

impl SerialParsable for Document {
    
    fn serial_parse(&mut self, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError> {

        if let Some(mut chapters) = std::mem::take(&mut self.chapters) {
            for chapter in chapters.iter_mut() {
                let _ = chapter.parse(Arc::clone(&parsing_configuration))?;
            }
        }

        Ok(())
    }
}

impl Parsable for Document {

    fn parse(&mut self, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError> {

        log::info!("parsing {} chapters of document: '{}' (parallelization level: {:?})", self.n_chapters(), self.name, parsing_configuration.parallelization_level());

        if *parsing_configuration.parallelization_level() >= ParallelizationLevel::Medium {

            self.parallel_parse(Arc::clone(&parsing_configuration))?;

        } else {
            self.serial_parse(Arc::clone(&parsing_configuration))?;
            
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
        
        let chapters: Option<Vec<Chapter>> = Option::None;

        if !content.is_empty() {
            todo!()
        }

        Self {
            name,
            chapters
        }
    }

    pub fn chapters(&self) -> &Option<Vec<Chapter>> {
        &self.chapters
    }

    pub fn n_chapters(&self) -> usize {
        if let Some(chapters) = self.chapters() {
            return chapters.len()
        }

        0
    }
}