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

pub use self::chapter::Paragraph;
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
    preamble: Option<Paragraph>,
    chapters: Vec<Chapter>
}


#[allow(dead_code)]
impl Document {

    pub fn new(name: String, preamble: Option<Paragraph>, chapters: Vec<Chapter>) -> Self {
        
        Self {
            name,
            preamble,
            chapters
        }
    }

    pub fn chapters(&self) -> &Vec<Chapter> {
        &self.chapters
    }

    pub fn preamble(&self) -> &Option<Paragraph> {
        &self.preamble
    }
}


impl Document {
    
    fn load_content_from_str(&mut self, codex: Arc<Codex>, content: &str) -> Result<(), DocumentError> {

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

            self.preamble = Option::Some(Paragraph::from(preamble));

            return Ok(())
        }

        let end_preamble = end_preamble.unwrap();

        let mut document_chapters: Vec<Chapter> = Vec::new();

        let mut chapter_builder: Option<ChapterBuilder> = Option::None;

        for (_, line) in content.lines().enumerate().filter(|(index, _)| *index >= end_preamble) {
            
            if Modifier::is_heading(line) {
                
                if let Some(chapter_builder) = chapter_builder {
                    document_chapters.push(chapter_builder.build()?)
                }

                chapter_builder = Option::Some(ChapterBuilder::new_with_heading(Arc::clone(&codex), line.to_string()));

            } else {

                let mut line = line.to_string();
                line.push_str("\n");        // because .lines() remove \n

                if let Some(ref mut chapter_builder) = chapter_builder {
                    chapter_builder.append_content(line);
                }
            }
        }

        if let Some(chapter_builder) = chapter_builder {
            document_chapters.push(chapter_builder.build()?)
        }

        if !preamble.is_empty() {
            self.preamble = Option::Some(Paragraph::from(preamble));
        }

        self.chapters = document_chapters;

        Ok(())
    }
}

impl Loadable<String> for Document {

    fn load(codex: Arc<Codex>, location: &String) -> Result<Box<Self>, LoadError> {

        let path_buf = PathBuf::from_str(&location).unwrap();

        let resource = DiskResource::try_from(path_buf)?;

        Self::load(Arc::clone(&codex), &resource)
    }
}

impl Loadable<DiskResource> for Document {

    fn load(codex: Arc<Codex>, resource: &DiskResource) -> Result<Box<Self>, LoadError> {
        let content = resource.content()?;

        let document_name = resource.name();

        let mut document = Box::new(Self {
            name: document_name.clone(),
            preamble: Option::None,
            chapters: Vec::new()
        });

        if content.is_empty() {
            return Ok(document);
        }

        match document.load_content_from_str(Arc::clone(&codex), &content) {
            Ok(_) => {
                return Ok(document);
            },
            Err(err) => return Err(LoadError::ElaborationError(err.to_string()))
        }
    }
}


impl Parsable for Document {

    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError> {

        log::info!("parsing {} chapters of document: '{}'", self.chapters().len(), self.name);

        if let Some(p) = &mut self.preamble {
            p.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration))?;
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
            s.push_str(preamble.to_string().as_str());
        }

        for chapter in &self.chapters {
            s.push_str(chapter.to_string().as_str());
        }

        write!(f, "{}", s)
    }
}


#[cfg(test)]
mod test {
    use crate::compiler::parsable::codex::codex_configuration::CodexConfiguration;

    use super::*;

    #[test]
    fn chapters_creation() {

        let codex = Codex::of_html(CodexConfiguration::default());

        let content: String = 
r#"
# title 1a

paragraph 1a

## title 2a

paragraph 2a

# title 1b

paragraph 1b
"#.trim().to_string();

        let mut document = Box::new(Document {
            name: "test document".to_string(),
            preamble: Option::None,
            chapters: Vec::new()
        });

        document.load_content_from_str(Arc::new(codex), &content).unwrap();

        assert!(document.preamble().is_none());

        assert_eq!(document.chapters().len(), 3);


        
    }
}