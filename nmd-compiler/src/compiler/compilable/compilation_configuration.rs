use std::str::FromStr;

use crate::compiler::{parsable::{codex::{parsing_rule::parsing_result::{ParsingError, ParsingOutcome}, Codex}, Parsable, ParsingConfiguration}, supported_format::SupportedFormat, resource::Resource};


pub struct CompilationConfiguration {
    format: SupportedFormat,
    codex: Codex,
    output: Resource,
    parsing_configuration: ParsingConfiguration,
    strict: bool
}

impl Default for CompilationConfiguration {
    fn default() -> Self {
        Self {
            format: SupportedFormat::Html,
            codex: Codex::of_html(),
            output: Resource::from_str(".").unwrap(),        // TODO
            parsing_configuration: ParsingConfiguration::default(),
            strict: false
        }
    }
}

impl CompilationConfiguration {
    pub fn new() -> Self {
        CompilationConfiguration {      // TODO
            ..Default::default()
        }
    }

    pub fn codex(&self) -> &Codex {
        &self.codex
    }
}