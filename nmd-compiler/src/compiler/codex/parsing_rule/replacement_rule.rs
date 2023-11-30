use log::debug;
use regex::Regex;

use crate::compiler::codex::{parsable::ParsingConfiguration, ParsingResultBody, parsing_result::ParsingError};

use super::{PatternType, ParsingRule, ParsingResult};


/// Rule to parse a NMD text based on a specific pattern matching rule
pub struct ReplacementRule {
    pattern_type: PatternType, 
    search_pattern: &'static str,     // TODO: maybe &str
    replacement_pattern: &'static str
}

impl ReplacementRule {
    /// Returns a new instance having a search pattern and a replication pattern
    ///
    /// # Arguments
    ///
    /// * `search_pattern` - A string slice which represent the pattern used to search in text 
    /// * `replacement_pattern` - A string slice which represent the pattern used to replace the text
    ///
    pub fn new(pattern_type: PatternType, search_pattern: &'static str, replacement_pattern: &'static str) -> Self {

        debug!("created new parsing rule with search_pattern: '{}' and replacement_pattern: '{}'", search_pattern, replacement_pattern);

        ReplacementRule {
            pattern_type,
            search_pattern,
            replacement_pattern
        }
    }
}

impl ParsingRule for ReplacementRule {

    /// Parse the content using internal search and replacement pattern
    fn parse(&self, content: &str, parsing_configuration: ParsingConfiguration) -> ParsingResult {

        let regex = match Regex::new(&self.search_pattern) {
          Ok(r) => r,
          Err(_) => return Err(ParsingError::InvalidPattern(self.search_pattern))  
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
        // valid pattern with a valid text
        let parsing_rule = ParsingRule::new(PatternType::Bold, r"\*\*(.*?)\*\*", "<strong>$1</strong>");

        let text_to_parse = r"A piece of **bold text**";

        let parsed_text = parsing_rule.parse(text_to_parse).unwrap().parsed_content();

        assert_eq!(parsed_text, r"A piece of <strong>bold text</strong>");

        // with invalid text
        let text_to_parse = r"A piece of text without bold text";

        let parsed_text = parsing_rule.parse(text_to_parse).unwrap().parsed_content();

        assert_eq!(parsed_text, r"A piece of text without bold text");
    }
}