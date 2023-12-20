use std::{str::FromStr, sync::Arc, path::PathBuf};

use crate::compiler::{parsable::{codex::{parsing_rule::parsing_result::{ParsingError, ParsingOutcome}, Codex, codex_configuration::CodexConfiguration}, Parsable, ParsingConfiguration}, supported_format::SupportedFormat, resource::Resource, assembler::{Assembler, self, assembler_configuration::AssemblerConfiguration}, dossier::dossier_configuration::DossierConfiguration, portability_level::PortabilityLevel};

pub struct CompilationConfiguration {
    format: SupportedFormat,
    output_location: PathBuf,
    portability_level: PortabilityLevel,
    dossier_configuration: DossierConfiguration
}

impl Default for CompilationConfiguration {
    fn default() -> Self {
        Self {
            format: SupportedFormat::Html,
            output_location: PathBuf::default(),        // TODO
            portability_level: PortabilityLevel::AllInOne,
            dossier_configuration: DossierConfiguration::default()
        }
    }
}

impl CompilationConfiguration {
    pub fn new() -> Self {
        CompilationConfiguration {      // TODO
            ..Default::default()
        }
    }

    pub fn codex(&self) -> Codex {
        Codex::from(&self.format, CodexConfiguration::default())
    }

    pub fn parsing_configuration(&self) -> ParsingConfiguration {
        ParsingConfiguration::from(self.portability_level.clone())
    }

    pub fn dossier_configuration(&self) -> &DossierConfiguration {
        &self.dossier_configuration
    }

    pub fn output_location(&self) -> &PathBuf {
        &self.output_location
    }

    pub fn assembler(&self) -> Box<dyn Assembler> {
        assembler::from(self.format.clone(), AssemblerConfiguration::new(self.output_location.clone(), self.portability_level.clone()))
    }
}