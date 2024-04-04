use std::sync::Arc;

use crate::compiler::codex::Codex;

use super::parsing_rule::{parsing_configuration::ParsingConfiguration, parsing_error::ParsingError};

pub trait Parsable {
    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError>;
}