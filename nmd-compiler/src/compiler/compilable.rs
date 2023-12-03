use super::parsable::{Parsable, ParsingConfiguration};
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

pub struct CompilationConfiguration {
    parsing_configuration: ParsingConfiguration
}

impl Default for CompilationConfiguration {
    fn default() -> Self {
        Self { parsing_configuration: Default::default() }
    }
}

impl CompilationConfiguration {
    pub fn new() -> Self {
        CompilationConfiguration { 
            ..Default::default()
        }
    }
}