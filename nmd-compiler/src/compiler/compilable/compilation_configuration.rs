use std::{str::FromStr, sync::Arc};

use crate::compiler::{parsable::{codex::{parsing_rule::parsing_result::{ParsingError, ParsingOutcome}, Codex}, Parsable, ParsingConfiguration}, supported_format::SupportedFormat, resource::Resource, assembler::{Assembler, self, assembler_configuration::AssemblerConfiguration}, dossier::dossier_configuration::DossierConfiguration, portability_level::PortabilityLevel};

pub struct CompilationConfiguration {
    format: SupportedFormat,
    output: Resource,
    portability_level: PortabilityLevel,
    dossier_configuration: DossierConfiguration
}

impl Default for CompilationConfiguration {
    fn default() -> Self {
        Self {
            format: SupportedFormat::Html,
            output: Resource::from_str(".").unwrap(),        // TODO
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
        Codex::from(self.format.clone())
    }

    pub fn parsing_configuration(&self) -> ParsingConfiguration {
        ParsingConfiguration::from(self.portability_level.clone())
    }

    pub fn dossier_configuration(&self) -> &DossierConfiguration {
        &self.dossier_configuration
    }

    pub fn assembler(&self) -> Box<dyn Assembler> {
        assembler::from(self.format.clone(), AssemblerConfiguration::from(self.portability_level.clone()))
    }
}