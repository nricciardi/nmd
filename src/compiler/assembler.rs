use thiserror::Error;

use self::{html_assembler::HtmlAssembler, assembler_configuration::AssemblerConfiguration};

use super::{artifact::{Artifact, ArtifactError}, dossier::Dossier, output_format::OutputFormat, parsable::codex::Codex};

pub mod html_assembler;
pub mod assembler_configuration;


#[derive(Error, Debug)]
pub enum AssemblerError {
    #[error("too few elements to assemble: {0}")]
    TooFewElements(String),

    #[error(transparent)]
    ArtifactError(#[from] ArtifactError),

    #[error(transparent)]
    ParseError(#[from] ),
}

pub trait Assembler {

    fn set_configuration(&mut self, configuration: AssemblerConfiguration);

    fn assemble(&self, codex: &Codex, dossier: Dossier) -> Result<Artifact, AssemblerError>;
}

pub fn from(format: OutputFormat, configuration: AssemblerConfiguration) -> Box<dyn Assembler> {
    match format {
        OutputFormat::Html => Box::new(HtmlAssembler::new(configuration))  
    }
}