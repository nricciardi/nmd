pub mod parsed_content_accessor;

use std::sync::{Arc, RwLock};

use crate::compiler::codex::Codex;

use super::parsing::{parsing_configuration::ParsingConfiguration, parsing_error::ParsingError, parsing_metadata::ParsingMetadata};


pub trait Parsable {
    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<(), ParsingError>;
}