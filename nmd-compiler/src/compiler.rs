mod compiler_configuration;
mod supported_format;
mod codex;
mod repository;
mod location;


use thiserror::Error;
pub use self::compiler_configuration::CompilerConfiguration;
use self::location::Locatable;


#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("unknown error")]
    Unknown
}

pub struct Compiler {
    version: &'static str,
    configuration: CompilerConfiguration
}

impl Compiler {

    pub fn new(configuration: CompilerConfiguration) -> Self {
        Compiler {
            version: "0.0.1",
            configuration
        }
    }

    pub fn compile(&self) -> Result<(), CompilerError> {
        todo!("compile...")
    }

    pub fn version(&self) -> &str {
        self.version
    }
}