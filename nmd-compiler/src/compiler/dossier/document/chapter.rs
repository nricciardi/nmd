pub mod paragraph;

pub use self::paragraph::Paragraph;
pub use crate::compiler::parsable::{codex::parsing_rule::{parsing_configuration::ParsingConfiguration, parsing_result::{ParsingError, ParsingOutcome}}, Parsable};

pub struct Chapter {
    heading: String,
    paragraphs: Option<Vec<Paragraph>>,
    subchapters: Option<Vec<Box<Chapter>>>
}

impl Parsable for Chapter {
    fn parse(&mut self, parsing_configuration: &ParsingConfiguration) -> Result<(), ParsingError> {
        todo!()
    }
}