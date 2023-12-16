use thiserror::Error;

use self::{html_assembler::HtmlAssembler, assembler_configuration::AssemblerConfiguration};

use super::{supported_format::SupportedFormat, dossier::Dossier, artifact::Artifact};

pub mod html_assembler;
pub mod assembler_configuration;


#[derive(Error, Debug)]
pub enum AssemblerError {
}

pub trait Assembler {

    fn set_configuration(&mut self, configuration: AssemblerConfiguration);

    fn assemble(&self, dossier: Dossier) -> Result<Artifact, AssemblerError>;
}

pub fn from(format: SupportedFormat, configuration: AssemblerConfiguration) -> Box<dyn Assembler> {
    match format {
        SupportedFormat::Html => Box::new(HtmlAssembler::new(configuration))  
    }
}