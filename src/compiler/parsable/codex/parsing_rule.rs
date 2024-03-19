pub mod replacement_rule;
pub mod parsing_outcome;
pub mod html_image_rule;
pub mod html_list_rule;
pub mod html_extended_block_quote_rule;

use super::modifier::{Modifier, modifiers_bucket::ModifiersBucket};
use std::sync::Arc;
use regex::Regex;
use crate::compiler::parsable::ParsingConfiguration;
use self::parsing_outcome::{ParsingOutcome, ParsingError};


pub trait ParsingRule: Send + Sync {

    fn search_pattern(&self) -> &String;

    fn is_match(&self, content: &str) -> bool {

        let pattern = self.search_pattern();

        let regex = Regex::new(&pattern).unwrap();

        regex.is_match(content)
    }

    fn parse(&self, content: &str, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError>;

    fn incompatible_modifiers(&self) -> &ModifiersBucket;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_heading() {
        let content = "#6 title 6";

        assert!(Modifier::heading_level(content).is_some());

        let content = "### title 3";

        assert!(Modifier::heading_level(content).is_some());

        let content = "text";

        assert!(Modifier::heading_level(content).is_none())
    }
}