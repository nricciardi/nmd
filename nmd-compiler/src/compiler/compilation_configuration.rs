use std::path::PathBuf;

use crate::compiler::{parsable::{codex::{Codex, codex_configuration::CodexConfiguration}, ParsingConfiguration}, output_format::OutputFormat};



pub struct CompilationConfiguration {
    format: OutputFormat,
    input_location: PathBuf,
    output_location: PathBuf
}

impl CompilationConfiguration {
    pub fn new(format: OutputFormat, input_location: PathBuf, output_location: PathBuf) -> Self {
        CompilationConfiguration {
            format,
            input_location,
            output_location
        }
    }

    pub fn format(&self) -> &OutputFormat {
        &self.format
    }

    pub fn codex(&self) -> Codex {
        Codex::from(&self.format, CodexConfiguration::default())
    }

    pub fn parsing_configuration(&self) -> ParsingConfiguration {

        let mut parsing_configuration = ParsingConfiguration::default();

        parsing_configuration.set_input_location(self.input_location.clone());
        parsing_configuration.set_output_location(self.output_location.clone());

        parsing_configuration
    }

    pub fn input_location(&self) -> &PathBuf {
        &self.input_location
    }

    pub fn output_location(&self) -> &PathBuf {
        &self.output_location
    }

    pub fn set_format(&mut self, format: OutputFormat) {
        self.format = format
    }

    pub fn set_input_location(&mut self, path: PathBuf) {
        self.input_location = path
    }

    pub fn set_output_location(&mut self, path: PathBuf) {
        self.output_location = path
    }
}

impl Default for CompilationConfiguration {
    fn default() -> Self {
        Self {
            format: Default::default(),
            input_location: PathBuf::from("."),
            output_location: PathBuf::from(".")
        }
    }
}