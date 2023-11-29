pub mod paragraph;

use crate::compiler::codex::parsable::Parsable;

pub use self::paragraph::Paragraph;

pub struct Chapter {
    heading: String,
    paragraphs: Option<Vec<Paragraph>>,
    subchapters: Option<Vec<Box<Chapter>>>
}

impl Parsable for Chapter {
    fn parse(&self, parsing_configuration: crate::compiler::codex::parsable::ParsingConfiguration) -> crate::compiler::codex::ParsingResult {
        todo!()
    }
}