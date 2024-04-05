pub mod chapter;

use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

pub use chapter::Chapter;
use thiserror::Error;
use log;
use rayon::prelude::*;

use crate::compiler::codex::Codex;
use crate::compiler::parser::parsable::Parsable;
use crate::compiler::parser::parsing_rule::parsing_configuration::ParsingConfiguration;
use crate::compiler::parser::parsing_rule::parsing_error::ParsingError;
use crate::resource::disk_resource::DiskResource;
use crate::resource::{Resource, ResourceError};

use self::chapter::paragraph::ParagraphError;
pub use self::chapter::Paragraph;
use self::chapter::chapter_builder::{ChapterBuilder, ChapterBuilderError};


#[derive(Error, Debug)]
pub enum DocumentError {
    #[error(transparent)]
    Load(#[from] ResourceError),

    #[error(transparent)]
    Parsing(#[from] ParsingError),

    #[error(transparent)]
    ChapterBuilderError(#[from] ChapterBuilderError),

    #[error(transparent)]
    ParagraphError(#[from] ParagraphError),
}

pub struct Document {
    name: String,
    preamble: Vec<Paragraph>,
    chapters: Vec<Chapter>
}


#[allow(dead_code)]
impl Document {

    pub fn new(name: String, preamble: Vec<Paragraph>, chapters: Vec<Chapter>) -> Self {
        
        Self {
            name,
            preamble,
            chapters
        }
    }

    pub fn chapters(&self) -> &Vec<Chapter> {
        &self.chapters
    }

    pub fn preamble(&self) -> &Vec<Paragraph> {
        &self.preamble
    }
}

// impl Loadable<String> for Document {

//     fn load(codex: Arc<Codex>, location: &String) -> Result<Box<Self>, LoadError> {

//         let path_buf = PathBuf::from_str(&location).unwrap();

//         let resource = DiskResource::try_from(path_buf)?;

//         Self::load(Arc::clone(&codex), &resource)
//     }
// }

// impl Loadable<DiskResource> for Document {

//     fn load(codex: Arc<Codex>, resource: &DiskResource) -> Result<Box<Self>, LoadError> {
//         let content = resource.content()?;

//         let document_name = resource.name();

//         match codex.load_document_from_str(document_name, &content) {
//             Ok(document) => {
//                 return Ok(Box::new(document))
//             },
//             Err(err) => return Err(LoadError::ElaborationError(err.to_string()))
//         }
//     }
// }


impl Parsable for Document {

    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError> {

        log::info!("parsing {} chapters of document: '{}'", self.chapters().len(), self.name);

        if parsing_configuration.parallelization() {

            // p.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration))?;

            let maybe_one_failed: Option<Result<(), ParsingError>> = self.preamble.par_iter_mut()
                .map(|paragraph| {

                    paragraph.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration))
                
                }).find_any(|result| result.is_err());

            if let Some(result) = maybe_one_failed {
                return result;
            }

            let maybe_one_failed: Option<Result<(), ParsingError>> = self.chapters.par_iter_mut()
                .map(|chapter| {

                    chapter.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration))
                
                }).find_any(|result| result.is_err());

            if let Some(result) = maybe_one_failed {
                return result;
            }
        
        } else {

            let maybe_one_failed: Option<Result<(), ParsingError>> = self.preamble.iter_mut()
                .map(|paragraph| {

                    paragraph.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration))
                
                }).find(|result| result.is_err());

            if let Some(result) = maybe_one_failed {
                return result;
            }
            
            let maybe_one_failed: Option<Result<(), ParsingError>> = self.chapters.iter_mut()
                .map(|chapter| {

                    chapter.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration))
                
                }).find(|result| result.is_err());

            if let Some(result) = maybe_one_failed {
                return result;
            }
        }

       Ok(())

    }
}


// impl Parsable for Document {

//     fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError> {

//         log::info!("parsing {} chapters of document: '{}'", self.chapters().len(), self.name);

//         let mut parsing_outcome = ParsingOutcome::new_empty();

//         if parsing_configuration.parallelization() {

//             let maybe_one_failed = self.preamble.par_iter_mut()
//                 .map(|paragraph| {

//                     let result = paragraph.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration));

//                     if let Ok(result) = result {
//                         parsing_outcome.append_parsed_content(&result.parsed_content())
//                     }

//                     result.map(|r| ())
                    
//                 }).find_any(|result| result.is_err());

//             if let Some(result) = maybe_one_failed {
//                 return Err(result.err().unwrap());
//             }

//             let maybe_one_failed = self.chapters.par_iter_mut()
//                 .map(|chapter| {

//                     let result = chapter.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration));

//                     if let Ok(result) = result {
//                         parsing_outcome.append_parsed_content(&result.parsed_content())
//                     }

//                     result.map(|r| ())
                    
//                 }).find_any(|result| result.is_err());

//             if let Some(result) = maybe_one_failed {
//                 return Err(result.err().unwrap());
//             }
        
//         } else {

//             let maybe_one_failed = self.preamble.iter_mut()
//                 .map(|paragraph| {

//                     let result = paragraph.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration));

//                     if let Ok(result) = result {
//                         parsing_outcome.append_parsed_content(&result.parsed_content())
//                     }

//                     result.map(|r| ())
                    
//                 }).find(|result| result.is_err());

//             if let Some(result) = maybe_one_failed {
//                 return Err(result.err().unwrap());
//             }

//             let maybe_one_failed = self.chapters.iter_mut()
//                 .map(|chapter| {

//                     let result = chapter.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration));

//                     if let Ok(result) = result {
//                         parsing_outcome.append_parsed_content(&result.parsed_content())
//                     }

//                     result.map(|r| ())
                    
//                 }).find(|result| result.is_err());

//             if let Some(result) = maybe_one_failed {
//                 return Err(result.err().unwrap());
//             }
//         }

//        Ok(parsing_outcome)

//     }
// }

// impl Display for Document {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

//         let mut s = String::new();

//         for paragraph in &self.preamble {
//             s.push_str(paragraph.to_string().as_str());
//         }

//         for chapter in &self.chapters {
//             s.push_str(chapter.to_string().as_str());
//         }

//         write!(f, "{}", s)
//     }
// }


#[cfg(test)]
mod test {

    use crate::compiler::codex::codex_configuration::CodexConfiguration;

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