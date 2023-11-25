mod compiler_configuration;
mod supported_format;
mod codex;
mod repository;
mod location;
mod parsable;
mod compilable;

use thiserror::Error;
pub use self::compiler_configuration::CompilerConfiguration;
use self::{compilable::Compilable, location::LocationError};


#[derive(Error, Debug)]
pub enum CompilerError {
    #[error(transparent)]
    InvalidTarget(#[from] LocationError),

    #[error("unknown error")]
    Unknown(String)
}

pub struct Compiler {
    version: &'static str,
    configuration: CompilerConfiguration,
    target: Box<dyn Compilable>
}

impl Compiler {

    pub fn new(configuration: CompilerConfiguration) -> Result<Self, CompilerError> {

        let target = configuration.location().load()?;

        Ok(Compiler {
            version: "0.0.1",
            configuration,
            target
        })
    }

    pub fn compile(&self) -> Result<(), CompilerError> {
        todo!("compile...")
    }

    pub fn version(&self) -> &str {
        self.version
    }
}