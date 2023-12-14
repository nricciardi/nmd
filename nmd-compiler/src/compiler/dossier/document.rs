pub mod chapter;

use std::str::FromStr;
use std::sync::{Arc, RwLock};

pub use chapter::Chapter;
use regex::Regex;
use thiserror::Error;
use log;
use rayon::prelude::*;

use crate::compiler::dossier::document;
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
    preamble: Option<String>,
    chapters: Option<Vec<Chapter>>
}


impl Document {
    fn get_document_body_from_str(content: &str) -> (Option<String>, Option<Vec<Chapter>>) {
        let mut preamble: String = String::new();
        
        let mut end_preamble: Option<usize> = Option::None;
        for (index, line) in content.lines().enumerate() {

            if !Modifier::is_heading(line) {
                preamble.push_str(line);
            } else {
                end_preamble = Some(index);
                break;
            }
        }

        if end_preamble.is_none() {
            return (Option::Some(preamble), Option::None);
        }

        let end_preamble = end_preamble.unwrap();

        // TODO
        let mut chapters: Vec<Chapter> = Vec::new();
        let mut current_chapter: Option<Chapter> = Option::None;

        for line in content.lines().enumerate().filter(|(index, line)| *index >= end_preamble) {
            todo!()
        }
        
        let mut result: (Option<String>, Option<Vec<Chapter>>) = (Option::None, Option::None);

        if !preamble.is_empty() {
            result.0 = Option::Some(preamble);
        }

        if !chapters.is_empty() {
            result.1 = Option::Some(chapters)
        }

        result
    }
}

impl Loadable for Document {

    type Type = Resource;

    fn load(resource: Self::Type) -> Result<Box<Self>, LoadError> {
        let content = resource.content()?;

        let document_name = resource.name();

        if content.is_empty() {
            return Ok(Box::new(Self {
                name: document_name.clone(),
                preamble: Option::None,
                chapters: Option::None
            }));
        }

        let (preamble, chapters) = Self::get_document_body_from_str(&content);

        Ok(Box::new(Self {
            name: document_name.clone(),
            preamble,
            chapters
        }))
    }
}


impl Parsable for Document {

    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError> {

        log::info!("parsing {} chapters of document: '{}'", self.n_chapters(), self.name);

        if let Some(p) = &self.preamble {
            self.preamble = Option::Some(codex.parse(p, Arc::clone(&parsing_configuration))?.parsed_content());
        }

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

    pub fn new(name: String, preamble: Option<String>, chapters: Option<Vec<Chapter>>) -> Self {
        
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
# title 1a

paragraph 1a

## title 2a

paragraph 2a

# title 1b

paragraph 1b
"#.trim().to_string();

        let (preamble, chapters) = Document::get_document_body_from_str(&content);

        assert!(preamble.is_none());

        let chapters = chapters.unwrap();

        assert_eq!(chapters.len(), 3);


        
    }
}