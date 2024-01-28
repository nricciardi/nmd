pub mod chapter;

use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

pub use chapter::Chapter;
use thiserror::Error;
use log;
use rayon::prelude::*;

use crate::compiler::parsable::codex::{Modifier, Codex};
use crate::compiler::parsable::{ParsingError, Parsable};
use crate::compiler::parsable::parsing_configuration::ParsingConfiguration;
use crate::compiler::loadable::{Loadable, LoadError};
use crate::compiler::resource::disk_resource::DiskResource;
use crate::compiler::resource::{Resource, ResourceError};

use self::chapter::chapter_builder::{ChapterBuilder, ChapterBuilderError};


#[derive(Error, Debug)]
pub enum DocumentError {
    #[error(transparent)]
    Load(#[from] ResourceError),

    #[error(transparent)]
    Parsing(#[from] ParsingError),

    #[error(transparent)]
    ChapterBuilderError(#[from] ChapterBuilderError)
}

pub struct Document {
    name: String,
    preamble: Option<String>,
    chapters: Vec<Chapter>
}

impl Document {

    pub fn new(name: String, preamble: Option<String>, chapters: Vec<Chapter>) -> Self {
        
        Self {
            name,
            preamble,
            chapters
        }
    }

    pub fn chapters(&self) -> &Vec<Chapter> {
        &self.chapters
    }

    pub fn preamble(&self) -> &Option<String> {
        &self.preamble
    }
}


impl Document {
    // TODO: change method signature
    fn get_document_body_from_str(content: &str) -> Result<(Option<String>, Vec<Chapter>), DocumentError> {
        let mut preamble: String = String::new();
        
        let mut end_preamble: Option<usize> = Option::None;
        for (index, line) in content.lines().enumerate() {

            if Modifier::heading_level(line).is_none() {
                preamble.push_str(line);
            } else {
                end_preamble = Some(index);
                break;
            }
        }

        if end_preamble.is_none() {     // => there is no chapters
            return Ok((Option::Some(preamble), Vec::new()));
        }

        let end_preamble = end_preamble.unwrap();

        let mut document_chapters: Vec<Chapter> = Vec::new();

        let mut chapter_builder: Option<ChapterBuilder> = Option::None;
        for (_, line) in content.lines().enumerate().filter(|(index, _)| *index >= end_preamble) {
            
            if Modifier::is_heading(line) {
                
                if let Some(chapter_builder) = chapter_builder {
                    document_chapters.push(chapter_builder.build()?)
                }

                chapter_builder = Option::Some(ChapterBuilder::new_with_heading(line.to_string()));

            } else {
                if let Some(ref mut chapter_builder) = chapter_builder {
                    chapter_builder.append_content(line.to_string());
                }
            }
        }

        if let Some(chapter_builder) = chapter_builder {
            document_chapters.push(chapter_builder.build()?)
        }
        
        
        let mut result: (Option<String>, Vec<Chapter>) = (Option::None, document_chapters);

        if !preamble.is_empty() {
            result.0 = Option::Some(preamble);
        }

        Ok(result)
    }
}

impl Loadable<String> for Document {

    fn load(location: &String) -> Result<Box<Self>, LoadError> {

        let path_buf = PathBuf::from_str(&location).unwrap();

        let resource = DiskResource::try_from(path_buf)?;

        Self::load(&resource)
    }
}

impl Loadable<DiskResource> for Document {

    fn load(resource: &DiskResource) -> Result<Box<Self>, LoadError> {
        let content = resource.content()?;

        let document_name = resource.name();

        if content.is_empty() {
            return Ok(Box::new(Self {
                name: document_name.clone(),
                preamble: Option::None,
                chapters: Vec::new()
            }));
        }

        let result = Self::get_document_body_from_str(&content);

        match result {
            Ok((preamble, chapters)) => {
                return Ok(Box::new(Self {
                    name: document_name.clone(),
                    preamble,
                    chapters
                }));
            },
            Err(err) => return Err(LoadError::ElaborationError(err.to_string()))
        }
    }
}


impl Parsable for Document {

    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError> {

        log::info!("parsing {} chapters of document: '{}'", self.chapters().len(), self.name);

        if let Some(p) = &self.preamble {
            self.preamble = Option::Some(codex.parse(p, Arc::clone(&parsing_configuration))?.parsed_content());
        }


        let maybe_one_failed = self.chapters.par_iter_mut()
            .map(|chapter| {

                chapter.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration))
            
            }).find_any(|result| result.is_err());

        if let Some(result) = maybe_one_failed {
            return result;
        }

       Ok(())

    }
}

impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let mut s = String::new();

        if let Some(preamble) = self.preamble() {
            s.push_str(preamble);
        }

        for chapter in &self.chapters {
            s.push_str(chapter.to_string().as_str());
        }

        write!(f, "{}", s)
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

        let (preamble, chapters) = Document::get_document_body_from_str(&content).unwrap();

        assert!(preamble.is_none());

        let chapters = chapters;

        assert_eq!(chapters.len(), 3);


        
    }
}