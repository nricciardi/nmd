use crate::compiler::parsable::{Parsable, ParsingConfiguration, ParsingResult, parsing_result::ParsingResultBody};



pub struct Paragraph {
    content: String
}

impl Parsable for Paragraph {
    fn parse(&self, parsing_configuration: &ParsingConfiguration) -> ParsingResult {

        let mut parsed_content = self.content.clone();

        for rule in parsing_configuration.codex().rules() {
            parsed_content = rule.parse(&parsed_content, parsing_configuration)?.parsed_content();
        }

        Ok(ParsingResultBody::new(parsed_content))
    }
}