use log::debug;
use regex::Regex;

use super::parsing_configuration::ParsingConfiguration;
use super::parsing_result::{ParsingError, ParsingOutcome};
use super::{PatternType, ParsingRule};


/// Rule to replace a NMD text based on a specific pattern matching rule
pub struct ReplacementRule {
    pattern_type: PatternType,
    replacement_pattern: &'static str
}

impl ReplacementRule {
    /// Returns a new instance having a search pattern and a replication pattern
    ///
    /// # Arguments
    ///
    /// * `pattern_type` - PatternType which represent the pattern used to search in text 
    /// * `replacement_pattern` - A string slice which represent the pattern used to replace the text
    ///
    pub fn new(pattern_type: PatternType, replacement_pattern: &'static str) -> Self {

        debug!("created new parsing rule with search_pattern: '{}' and replacement_pattern: '{}'", pattern_type.search_pattern(), replacement_pattern);

        ReplacementRule {
            pattern_type,
            replacement_pattern
        }
    }
}

impl ParsingRule for ReplacementRule {

    /// Parse the content using internal search and replacement pattern
    fn parse(&self, content: &str, parsing_configuration: &ParsingConfiguration) -> Result<ParsingOutcome, ParsingError> {

        let regex = match Regex::new(&self.pattern_type.search_pattern()) {
          Ok(r) => r,
          Err(_) => return Err(ParsingError::InvalidPattern(self.pattern_type.search_pattern()))  
        };

        let parsed_content = regex.replace_all(content, self.replacement_pattern).to_string();

        debug!("parsed '{}' using '{}'->'{}'", content, self.pattern_type.search_pattern(), self.replacement_pattern);
        
        Ok(ParsingOutcome::new(parsed_content))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn parsing() {
        // valid pattern with a valid text modifier
        let parsing_rule = ReplacementRule::new(PatternType::BoldV1, "<strong>$1</strong>");

        let text_to_parse = r"A piece of **bold text**";
        let parsing_configuration = ParsingConfiguration::default();

        let parsed_text = parsing_rule.parse(text_to_parse, &parsing_configuration).unwrap().parsed_content();

        assert_eq!(parsed_text, r"A piece of <strong>bold text</strong>");

        // without text modifier
        let text_to_parse = r"A piece of text without bold text";

        let parsed_text = parsing_rule.parse(text_to_parse, &parsing_configuration).unwrap().parsed_content();

        assert_eq!(parsed_text, r"A piece of text without bold text");

        // headings
        let parsing_rule = ReplacementRule::new(PatternType::HeadingH6, "<h6>$1</h6>");

        let text_to_parse = r"###### title 6";
        let parsing_configuration = ParsingConfiguration::default();

        let parsed_text = parsing_rule.parse(text_to_parse, &parsing_configuration).unwrap().parsed_content();

        assert_eq!(parsed_text, r"<h6>title 6</h6>");
    }
}