use std::str::FromStr;

use crate::compiler::{parsable::{ParsingConfiguration, parsing_configuration::Codex}, location::Location, supported_format::SupportedFormat};


pub enum ParallelizationLevel {     // TODO: maybe in ParsingConfiguration
    NoParallelization,
    DocumentParallelization,
    ChapterParallelization(i32)
}




pub enum Metadata {

}

pub struct CompilationConfiguration {
    parallelization_level: ParallelizationLevel,
    format: SupportedFormat,
    documents: Vec<Location>,
    styles: Vec<Location>,
    metadata: Vec<Metadata>,
    output: Location,
    parsing_configuration: ParsingConfiguration
}

impl Default for CompilationConfiguration {
    fn default() -> Self {
        Self {
            parallelization_level: ParallelizationLevel::NoParallelization,
            format: SupportedFormat::Html,
            documents: vec![],          // TODO: all .nmd file in running directory
            styles: vec![],              // TODO: default style
            metadata: vec![],
            output: Location::from_str(".").unwrap(),        // TODO
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