pub mod replacement_rule;
pub mod html_image_rule;
pub mod html_list_rule;
pub mod html_extended_block_quote_rule;
pub mod html_greek_letter_rule;
pub mod reference_rule;
pub mod html_table_rule;
pub mod constants;

use std::{fmt::Debug, sync::{Arc, RwLock}};
use regex::{Match, Matches, Regex};

use crate::compiler::codex::Codex;

use super::{parsing_configuration::ParsingConfiguration, parsing_error::ParsingError, parsing_outcome::ParsingOutcome};


pub trait ParsingRule: Send + Sync + Debug {

    fn search_pattern(&self) -> &String;

    fn search_pattern_regex(&self) -> &Regex;

    fn is_match(&self, content: &str) -> bool {

        self.search_pattern_regex().is_match(content)
    }

    fn find_iter<'r, 'h>(&'r self, content: &'h str) -> Vec<Match<'h>> {
        self.search_pattern_regex().find_iter(content).collect()
    }

    /// Parse content based on `Codex` and `ParsingConfiguration`
    fn standard_parse(&self, content: &str, codex: &Codex, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<ParsingOutcome, ParsingError>;

    /// Parse content based on `Codex` and `ParsingConfiguration` avoid time consuming operations. This is an incomplete parsing
    fn fast_parse(&self, content: &str, codex: &Codex, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<ParsingOutcome, ParsingError> {
        self.standard_parse(content, codex, parsing_configuration)
    }

    /// Standard or fast parse based on `ParsingConfiguration` `fast_draft()`
    fn parse(&self, content: &str, codex: &Codex, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<ParsingOutcome, ParsingError> {
        if parsing_configuration.read().unwrap().fast_draft() {
            return self.fast_parse(content, codex, parsing_configuration)
        }

        self.standard_parse(content, codex, parsing_configuration)
    }


}