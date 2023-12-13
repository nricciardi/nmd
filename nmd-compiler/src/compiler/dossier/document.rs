pub mod chapter;

use std::sync::{Arc, RwLock};

pub use chapter::Chapter;
use regex::Regex;
use thiserror::Error;
use log;
use rayon::prelude::*;

use crate::compiler::parsable::codex::{Modifier, Codex};
use crate::compiler::parsable::{ParsingError, Parsable};
use crate::compiler::parsable::parsing_configuration::{ParsingConfiguration};
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
    preamble: String,
    chapters: Option<Vec<Chapter>>
}

impl Loadable for Document {

    type Type = Resource;

    fn load(resource: Self::Type) -> Result<Box<Self>, LoadError> {
        let content = resource.content()?;

        todo!()
        /* Ok(Box::new(Self::new(resource.name().clone(), content))) */
    }
}

impl Parsable for Document {

    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError> {

        log::info!("parsing {} chapters of document: '{}'", self.n_chapters(), self.name);

        self.preamble = codex.parse(&self.preamble, Arc::clone(&parsing_configuration))?.parsed_content();

        if let Some(chapters) = &mut self.chapters {

            let maybe_one_failed = chapters.par_iter_mut()
           .map(|chapter| {

               chapter.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration))
           
           }).find_any(|result| result.is_err());
   
           if let Some(result) = maybe_one_failed {
               return result;
           }
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

    fn split_document(content: String) -> Option<Vec<String>> {

        if content.is_empty() {
            return Option::None;
        } 

        let heading_pattern = Modifier::heading_rules();

        /* let regex = Regex::new(format!("({})", heading_pattern.search_pattern()).as_str()).unwrap();

        let chapters: Vec<String> = regex.split(content.as_str()).into_iter().map(|chapter_content| {
            chapter_content.to_string()
        }).collect(); 
        
        Option::Some(chapters)*/

        todo!()
    }

    pub fn new(name: String, preamble: String, chapters: Option<Vec<Chapter>>) -> Self {
        
        Self {
            name,
            preamble,
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



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn chapters_creation() {

        let content: String = 
r#"
#1 title 1a

paragraph 1a

## title 2a

paragraph 2a

# title 1b

paragraph 1b
"#.trim().to_string();

        let chapters = Document::split_document(content);

        assert!(chapters.is_some());

        assert_eq!(chapters.unwrap().len(), 3);
    }
}