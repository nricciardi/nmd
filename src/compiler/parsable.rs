pub mod parsed_content_accessor;

use std::sync::{Arc, RwLock};

use crate::compiler::codex::Codex;

use super::{output_format::OutputFormat, parsing::{parsing_configuration::{parsing_configuration_overlay::ParsingConfigurationOverLay, ParsingConfiguration}, parsing_error::ParsingError}};


pub trait Parsable {

    /// Standard parse, using complete rules
    fn standard_parse(&mut self, format: &OutputFormat,  codex: Arc<Codex>, parsing_configuration: Arc<RwLock<ParsingConfiguration>>,
        parsing_configuration_overlay: Arc<Option<ParsingConfigurationOverLay>>) -> Result<(), ParsingError>;

    /// Fast parse, reduce parsing time, but its result is incomplete
    fn fast_parse(&mut self, format: &OutputFormat,  codex: Arc<Codex>, parsing_configuration: Arc<RwLock<ParsingConfiguration>>,
        parsing_configuration_overlay: Arc<Option<ParsingConfigurationOverLay>>) -> Result<(), ParsingError> {
            self.standard_parse(format, codex, parsing_configuration, parsing_configuration_overlay)
    }

    /// `standard_parse` or `fast_parse` based on parsing configuration `fast_draft()` value
    fn parse(&mut self, format: &OutputFormat, codex: Arc<Codex>, parsing_configuration: Arc<RwLock<ParsingConfiguration>>,
        parsing_configuration_overlay: Arc<Option<ParsingConfigurationOverLay>>) -> Result<(), ParsingError> {
            
        if parsing_configuration.read().unwrap().fast_draft() {
            return self.fast_parse(format, codex, parsing_configuration, parsing_configuration_overlay)
        }

        self.standard_parse(format, codex, parsing_configuration, parsing_configuration_overlay)
    }
}