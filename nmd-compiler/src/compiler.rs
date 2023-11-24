mod compiler_configuration;
mod supported_format;
mod codex;
mod repository;
mod location;
mod parsable;
mod compilable;

use thiserror::Error;
pub use self::compiler_configuration::CompilerConfiguration;
use self::compilable::Compilable;


#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("invalid target")]
    InvalidTarget(String),

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

        // TODO: improve with ?
        let target = match configuration.location().load() {
            Ok(target) => target,
            Err(err) => return Err(CompilerError::InvalidTarget(err.to_string()))
        };

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