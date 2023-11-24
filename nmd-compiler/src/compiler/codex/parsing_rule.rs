use log::{info, debug};
use regex::Regex;

pub use super::parsing_result::{ParsingResult, ParsingError, ParsingResultBody};




/// Rule to parse a NMD text based on a specific pattern matching rule
pub struct ParsingRule {
    search_pattern: &'static str,     // TODO: maybe &str
    replacement_pattern: &'static str
}

impl ParsingRule {
    pub fn new(search_pattern: &'static str, replacement_pattern: &'static str) -> Self {

        debug!("created new parsing rule with search_pattern: '{}' and replacement_pattern: '{}'", search_pattern, replacement_pattern);

        ParsingRule {
            search_pattern,
            replacement_pattern
        }
    }

    pub fn parse(self: &Self, content: &str) -> ParsingResult {

        let regex = match Regex::new(&self.search_pattern) {
          Ok(r) => r,
          Err(e) => return Err(ParsingError::InvalidPattern(self.search_pattern))  
        };

        let parsed_content = regex.replace_all(content, self.replacement_pattern).to_string();

        debug!("parsed '{}' using '{}'->'{}'", content, self.search_pattern, self.replacement_pattern);
        
        Ok(ParsingResultBody::new(parsed_content))
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn parsing() {
        let parsing_rule = ParsingRule::new(r"\*\*(.*?)\*\*", "<strong>$1</strong>");

        let text_to_parse = r"A piece of **bold text**";

        let parsed_text = parsing_rule.parse(text_to_parse).unwrap().parsed_content();

        assert_eq!(parsed_text, r"A piece of <strong>bold text</strong>")
    }
}