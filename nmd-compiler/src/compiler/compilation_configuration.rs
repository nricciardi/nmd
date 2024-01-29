use std::{str::FromStr, sync::Arc, path::PathBuf};

use crate::compiler::{parsable::{codex::{parsing_rule::parsing_outcome::{ParsingError, ParsingOutcome}, Codex, codex_configuration::CodexConfiguration}, Parsable, ParsingConfiguration}, output_format::OutputFormat, resource::Resource, assembler::{Assembler, self, assembler_configuration::AssemblerConfiguration}, dossier::dossier_configuration::{DossierConfiguration, self}};

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
        todo!()
    }

    pub fn input_location(&self) -> &PathBuf {
        &self.input_location
    }

    pub fn output_location(&self) -> &PathBuf {
        &self.output_location
    }
}