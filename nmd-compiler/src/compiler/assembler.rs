use thiserror::Error;

use self::{html_assembler::HtmlAssembler, assembler_configuration::AssemblerConfiguration};

use super::{output_format::OutputFormat, dossier::Dossier, artifact::{Artifact, ArtifactError}};

pub mod html_assembler;
pub mod assembler_configuration;


#[derive(Error, Debug)]
pub enum AssemblerError {
    #[error("too few elements to assemble: {0}")]
    TooFewElements(String),

    #[error(transparent)]
    ArtifactError(#[from] ArtifactError)
}

pub trait Assembler {

    fn set_configuration(&mut self, configuration: AssemblerConfiguration);

    fn assemble(&self, dossier: Dossier) -> Result<Artifact, AssemblerError>;
}

pub fn from(format: OutputFormat, configuration: AssemblerConfiguration) -> Box<dyn Assembler> {
    match format {
        OutputFormat::Html => Box::new(HtmlAssembler::new(configuration))  
    }
}