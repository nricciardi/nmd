pub mod paragraph;

use crate::compiler::parsable::Parsable;

pub use self::paragraph::Paragraph;
pub use crate::compiler::parsable::{ParsingConfiguration, ParsingResult};

pub struct Chapter {
    heading: String,
    paragraphs: Option<Vec<Paragraph>>,
    subchapters: Option<Vec<Box<Chapter>>>
}

impl Parsable for Chapter {
    fn parse(&self, parsing_configuration: &ParsingConfiguration) -> ParsingResult {
        todo!()
    }
}