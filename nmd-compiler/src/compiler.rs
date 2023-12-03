mod compiler_configuration;
mod supported_format;
mod location;
mod compilable;
mod parsable;
mod dossier;

use thiserror::Error;
pub use self::compiler_configuration::CompilerConfiguration;
use self::{compilable::{Compilable, CompilationError}, location::LocationError};


#[derive(Error, Debug)]
pub enum CompilerError {
    #[error(transparent)]
    InvalidTarget(#[from] LocationError),

    #[error(transparent)]
    CompilationError(#[from] CompilationError),

    #[error("unknown error")]
    Unknown(String)
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

    pub fn compile(&self) -> Result<(), CompilerError> {

        let target: Box<dyn Compilable> = self.configuration.location().load()?;

        Ok(target.compile(self.configuration.compilation_configuration())?)
    }

    pub fn version(&self) -> &str {
        self.version
    }
}
