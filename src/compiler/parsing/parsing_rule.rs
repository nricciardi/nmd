pub mod replacement_rule;
pub mod html_image_rule;
pub mod html_list_rule;
pub mod html_extended_block_quote_rule;
pub mod html_greek_letter_rule;
pub mod reference_rule;
pub mod html_table_rule;

use std::{fmt::Debug, sync::{Arc, RwLock}};
use regex::Regex;

use crate::compiler::codex::Codex;

use super::{parsing_configuration::ParsingConfiguration, parsing_error::ParsingError, parsing_outcome::ParsingOutcome};


pub trait ParsingRule: Send + Sync + Debug {

    fn searching_pattern(&self) -> &String;

    fn is_match(&self, content: &str) -> bool {

        let pattern = self.searching_pattern();

        let regex = Regex::new(&pattern).unwrap();

        regex.is_match(content)
    }

    /// Parse content based on codex and parsing_configuration.
    fn standard_parse(&self, content: &str, codex: &Codex, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<ParsingOutcome, ParsingError>;

    fn fast_parse(&self, content: &str, codex: &Codex, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<ParsingOutcome, ParsingError> {
        self.standard_parse(content, codex, parsing_configuration)
    }

    fn parse(&self, content: &str, codex: &Codex, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<ParsingOutcome, ParsingError> {
        if parsing_configuration.read().unwrap().fast_draft() {
            return self.fast_parse(content, codex, parsing_configuration)
        }

        self.standard_parse(content, codex, parsing_configuration)
    }


}