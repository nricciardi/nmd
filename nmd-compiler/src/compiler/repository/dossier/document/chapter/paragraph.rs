use crate::compiler::{codex::{parsable::{Parsable, ParsingConfiguration}, ParsingResult}, codex::{Codex, ParsingResultBody}};



pub struct Paragraph {
    content: String
}

impl Parsable for Paragraph {
    fn parse(&self, parsing_configuration: ParsingConfiguration) -> ParsingResult {

        let mut parsed_content = self.content.clone();

        for rule in parsing_configuration.codex().rules() {
            parsed_content = rule.parse(&parsed_content)?.parsed_content();
        }

        Ok(ParsingResultBody::new(parsed_content))
    }
}