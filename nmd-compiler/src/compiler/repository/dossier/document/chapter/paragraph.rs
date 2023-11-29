use crate::compiler::{parsable::{Parsable, ParsingConfiguration}, codex::Codex};



pub struct Paragraph {
    content: String
}

impl Parsable for Paragraph {
    fn parse(&self, parsing_configuration: ParsingConfiguration) {
        todo!()
    }
}