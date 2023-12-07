use crate::compiler::parsable::{codex::parsing_rule::{parsing_configuration::ParsingConfiguration, parsing_result::{ParsingError, ParsingOutcome}}, Parsable};



pub struct Paragraph {
    content: String
}

impl Parsable for Paragraph {
    fn parse(&mut self, parsing_configuration: &ParsingConfiguration) -> Result<(), ParsingError> {

        for rule in parsing_configuration.codex().rules() {
            self.content = rule.parse(&self.content, parsing_configuration)?.parsed_content();
        }

        Ok(())
    }
}