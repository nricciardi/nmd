use std::{str::FromStr, sync::Arc};

use crate::compiler::{parsable::{codex::{parsing_rule::parsing_result::{ParsingError, ParsingOutcome}, Codex}, Parsable, ParsingConfiguration}, supported_format::SupportedFormat, resource::Resource, assembler::{Assembler, self}, dossier::dossier_configuration::DossierConfiguration};


pub struct CompilationConfiguration {
    format: SupportedFormat,        // TODO: maybe remove
    codex: Arc<Codex>,
    assembler: Box<dyn Assembler>,
    output: Resource,
    parsing_configuration: Arc<ParsingConfiguration>,
    dossier_configuration: Arc<DossierConfiguration>
}

impl Default for CompilationConfiguration {
    fn default() -> Self {

        let format: SupportedFormat = SupportedFormat::Html;

        Self {
            format: format.clone(),
            codex: Arc::new(Codex::from(format.clone())),
            assembler: assembler::from(format.clone()),
            output: Resource::from_str(".").unwrap(),        // TODO
            parsing_configuration: Arc::new(ParsingConfiguration::default()),
            dossier_configuration: Arc::new(DossierConfiguration::default())
        }
    }
}

impl CompilationConfiguration {
    pub fn new() -> Self {
        CompilationConfiguration {      // TODO
            ..Default::default()
        }
    }

    pub fn codex(&self) -> Arc<Codex> {
        Arc::clone(&self.codex)
    }

    pub fn parsing_configuration(&self) -> Arc<ParsingConfiguration> {
        Arc::clone(&self.parsing_configuration)
    }

    pub fn dossier_configuration(&self) -> Arc<DossierConfiguration> {
        Arc::clone(&self.dossier_configuration)
    }

    pub fn assembler(&self) -> &Box<dyn Assembler> {
        &self.assembler
    }
}