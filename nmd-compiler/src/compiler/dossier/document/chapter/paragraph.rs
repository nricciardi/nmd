use std::sync::Arc;

use crate::compiler::parsable::{codex::{parsing_rule::parsing_result::ParsingError, Codex}, Parsable};
use crate::compiler::parsable::parsing_configuration::ParsingConfiguration;


pub struct Paragraph {
    content: String
}

impl Clone for Paragraph {
    fn clone(&self) -> Self {
        Self { content: self.content.clone() }
    }
}

impl Parsable for Paragraph {
    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError> {

        self.content = codex.parse(&self.content, Arc::clone(&parsing_configuration))?.parsed_content();

        Ok(())
    }
}