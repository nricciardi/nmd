use std::str::FromStr;

use crate::compiler::{parsable::{codex::{parsing_rule::{parsing_configuration::ParsingConfiguration, parsing_result::{ParsingError, ParsingOutcome}}, Codex}, Parsable}, supported_format::SupportedFormat, resource::Resource};





pub enum Metadata {

}

pub struct CompilationConfiguration {
    format: SupportedFormat,
    codex: Codex,
    documents: Vec<Resource>,
    styles: Vec<Resource>,
    metadata: Vec<Metadata>,
    output: Resource,
    parsing_configuration: ParsingConfiguration
}

impl Default for CompilationConfiguration {
    fn default() -> Self {
        Self {
            format: SupportedFormat::Html,
            codex: Codex::of_html(),
            documents: vec![],          // TODO: all .nmd file in running directory
            styles: vec![],              // TODO: default style
            metadata: vec![],
            output: Resource::from_str(".").unwrap(),        // TODO
            parsing_configuration: ParsingConfiguration::from(SupportedFormat::Html)
        }
    }
}

impl CompilationConfiguration {
    pub fn new() -> Self {
        CompilationConfiguration {      // TODO
            ..Default::default()
        }
    }
}