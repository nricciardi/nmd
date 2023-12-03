pub mod compilable_configuration;

use self::compilable_configuration::CompilationConfiguration;

use super::parsable::Parsable;
use super::location::Locatable;
use anyhow::Result;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum CompilationError {

    #[error("unknown error")]
    Unknown(String)
}

pub trait Compilable: Locatable + Parsable {
    fn compile(&self, compilation_configuration: &CompilationConfiguration) -> Result<(), CompilationError>;
}

