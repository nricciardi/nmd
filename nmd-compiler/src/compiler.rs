mod compilation_configuration;
mod parsable;
mod dossier;
pub mod output_format;
pub mod resource;
mod loadable;
mod assembler;
mod dumpable;
pub mod artifact;
mod utility; 

use std::sync::Arc;

use thiserror::Error;
use crate::compiler::{dossier::Dossier, loadable::Loadable};

use self::{compilation_configuration::CompilationConfiguration, loadable::LoadError, parsable::{Parsable, ParsingError}, assembler::AssemblerError};


#[derive(Error, Debug)]
pub enum CompilationError {
    /* #[error(transparent)]
    InvalidTarget(#[from] LocationError), */

    #[error("unknown error")]
    Unknown(String),

    #[error(transparent)]
    LoadError(#[from] LoadError),

    #[error(transparent)]
    ParsingError(#[from] ParsingError),

    #[error(transparent)]
    AssemblerError(#[from] AssemblerError)

}

pub struct Compiler {
}

impl Compiler {

    pub fn compile(&self, compilation_configuration: CompilationConfiguration) -> Result<(), CompilationError> {

        let mut dossier = Dossier::load(compilation_configuration.input_location())?;
        
        dossier.parse(Arc::new(compilation_configuration.codex()), Arc::new(compilation_configuration.parsing_configuration()))?;

        let artifact = compilation_configuration.assembler().assemble(*dossier)?;

        // TODO: dump

        todo!()
    }

    pub fn version(&self) -> &str {
        "0.0.1"
    }
}
