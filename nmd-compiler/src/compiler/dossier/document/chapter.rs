pub mod paragraph;

use std::sync::Arc;

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

pub use self::paragraph::Paragraph;
use crate::compiler::parsable::codex::Codex;
use crate::compiler::parsable::parsing_configuration::ParsingConfiguration;
use crate::compiler::parsable::{codex::parsing_rule::parsing_result::{ParsingError, ParsingOutcome}, Parsable};

pub struct Chapter {
    heading: String,
    paragraphs: Option<Vec<Paragraph>>,
    subchapters: Option<Vec<Box<Chapter>>>,
    superchapter: Option<Arc<Chapter>>
}

impl Chapter {

    pub fn new(heading: String, paragraphs: Option<Vec<Paragraph>>, subchapters: Option<Vec<Box<Chapter>>>, superchapter: Option<Arc<Chapter>>) -> Self {
        Self {
            heading,
            paragraphs,
            subchapters,
            superchapter
        }
    }

    pub fn new_empty() -> Self {
        Self { heading: String::new(), paragraphs: Option::None, subchapters: Option::None, superchapter: Option::None }
    }

    pub fn heading(&self) -> &String {
        &self.heading
    }

    pub fn set_heading(&mut self, heading: &String) -> () {
        self.heading = heading.clone()
    }

    pub fn paragraphs(&self) -> &Option<Vec<Paragraph>> {
        &self.paragraphs
    }

    pub fn n_paragraphs(&self) -> usize {
        if let Some(paragraphs) = &self.paragraphs {
            return paragraphs.len()
        }

        0
    }

    pub fn subchapters(&self) -> &Option<Vec<Box<Chapter>>> {
        &self.subchapters
    }

    pub fn n_subchapters(&self) -> usize {
        if let Some(subchapters) = &self.subchapters {
            return subchapters.len()
        }

        0
    }

    pub fn superchapter(&self) -> &Option<Arc<Chapter>> {
        &self.superchapter
    }
}

impl Clone for Chapter {
    fn clone(&self) -> Self {
        Self { heading: self.heading.clone(), paragraphs: self.paragraphs.clone(), subchapters: self.subchapters.clone(), superchapter: self.superchapter.clone() }
    }
}


impl Parsable for Chapter {
    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError> {
        self.heading = codex.parse(&self.heading, Arc::clone(&parsing_configuration))?.parsed_content();

        if let Some(paragraphs) = &mut self.paragraphs {
            let maybe_failed = paragraphs.par_iter_mut().map(|paragraph| {
                paragraph.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration))
            }).find_any(|result| result.is_err());

            if let Some(result) = maybe_failed {
                return result
            }
        }

        if let Some(mut subchapters) = std::mem::take(&mut self.subchapters) {
            let maybe_failed = subchapters.par_iter_mut().map(|subchapter| {
                subchapter.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration))
            }).find_any(|result| result.is_err());

            if let Some(result) = maybe_failed {
                return result
            }
        }

        Ok(())
    }
}