mod compiler_configuration;
mod supported_format;
mod codex;


use thiserror::Error;
pub use self::compiler_configuration::CompilerConfiguration;


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
}