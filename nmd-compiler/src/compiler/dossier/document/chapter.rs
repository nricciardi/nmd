pub mod paragraph;

use std::sync::{Arc, RwLock};

pub use self::paragraph::Paragraph;
use crate::compiler::parsable::parsing_configuration::{ParsingConfiguration, ParallelizationLevel};
use crate::compiler::parsable::{codex::parsing_rule::parsing_result::{ParsingError, ParsingOutcome}, Parsable};

pub struct Chapter {
    heading: String,
    paragraphs: Option<Vec<Paragraph>>,
    subchapters: Option<Vec<Box<Chapter>>>
}

impl Chapter {
    pub fn subchapters(&self) -> &Option<Vec<Box<Chapter>>> {
        &self.subchapters
    }
}

impl /* Parsable for */ Chapter {
    pub fn parse(&mut self, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError> {
        todo!()
    }
}