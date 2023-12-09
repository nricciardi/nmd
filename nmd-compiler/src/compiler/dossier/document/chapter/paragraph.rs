use crate::compiler::parsable::{codex::parsing_rule::parsing_result::{ParsingError, ParsingOutcome}, Parsable};
use crate::compiler::parsable::parsing_configuration::ParsingConfiguration;


pub struct Paragraph {
    content: String
}

impl Parsable for Paragraph {
    fn parse(&mut self, parsing_configuration: &ParsingConfiguration) -> Result<(), ParsingError> {

        todo!()
        /* for rule in parsing_configuration.codex().rules() {
            self.content = rule.parse(&self.content, parsing_configuration)?.parsed_content();
        }

        Ok(()) */
    }
}