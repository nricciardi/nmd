use log::debug;
use regex::Regex;

pub use super::parsing_result::{ParsingResult, ParsingError, ParsingResultBody};


/// NMD modifiers pattern types 
pub enum PatternType {
    Bold,
    Italic,
    Strikethrough,
    Underlined,
    Link,
    Image,
    Highlight,
    ColoredText,
    Emoji,
    Superscript,
    Subscript,
    InlineCode,
    Comment,
    Bookmark,
    Heading,
    CodeBlock,
    CommentBlock,
    FocusBlock,
    MathBlock,


    Custom
}


/// Rule to parse a NMD text based on a specific pattern matching rule
pub struct ParsingRule {
    pattern_type: PatternType, 
    search_pattern: &'static str,     // TODO: maybe &str
    replacement_pattern: &'static str
}

impl ParsingRule {

    /// Returns a new instance having a search pattern and a replication pattern
    ///
    /// # Arguments
    ///
    /// * `search_pattern` - A string slice which represent the pattern used to search in text 
    /// * `replacement_pattern` - A string slice which represent the pattern used to replace the text
    ///
    pub fn new(pattern_type: PatternType, search_pattern: &'static str, replacement_pattern: &'static str) -> Self {

        debug!("created new parsing rule with search_pattern: '{}' and replacement_pattern: '{}'", search_pattern, replacement_pattern);

        ParsingRule {
            pattern_type,
            search_pattern,
            replacement_pattern
        }
    }

    /// Parse the content using internal search and replacement pattern
    pub fn parse(self: &Self, content: &str) -> ParsingResult {

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