mod compiler_configuration;
mod compilable;
mod parsable;
mod dossier;
pub mod supported_format;
pub mod resource;
mod loadable;
mod assembler;
mod dumpable;
pub mod artifact;
pub mod portability_level;

use std::sync::Arc;

use thiserror::Error;
use crate::compiler::{dossier::Dossier, loadable::Loadable};

pub use self::compiler_configuration::CompilerConfiguration;
use self::{compilable::{CompilationError, compilation_configuration::CompilationConfiguration}, loadable::LoadError, parsable::{Parsable, ParsingError}, assembler::AssemblerError};


#[derive(Error, Debug)]
pub enum CompilerError {
    /* #[error(transparent)]
    InvalidTarget(#[from] LocationError), */

    #[error(transparent)]
    CompilationError(#[from] CompilationError),

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
    version: &'static str,
    configuration: CompilerConfiguration
}

impl Compiler {

    pub fn new(configuration: CompilerConfiguration) -> Result<Self, CompilerError> {

        Ok(Compiler {
            version: "0.0.1",
            configuration
        })
    }

    pub fn compile(&self, compilation_configuration: CompilationConfiguration) -> Result<(), CompilerError> {

        let mut dossier = Dossier::load(compilation_configuration.dossier_configuration())?;
        
        dossier.parse(Arc::new(compilation_configuration.codex()), Arc::new(compilation_configuration.parsing_configuration()))?;

        let artifact = compilation_configuration.assembler().assemble(*dossier)?;

        // TODO: dump

        todo!()
    }

    pub fn version(&self) -> &str {
        self.version
    }
}
