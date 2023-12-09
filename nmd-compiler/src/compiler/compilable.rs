pub mod compilation_configuration;

use self::compilation_configuration::CompilationConfiguration;

use super::{parsable::Parsable, loadable::{Loadable, LoadError}};
use anyhow::Result;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum CompilationError {

    #[error(transparent)]
    LoadError(#[from] LoadError),

    #[error("unknown error")]
    Unknown(String)
}

pub trait Compilable: Loadable + Parsable {
    fn compile(&self, compilation_configuration: &CompilationConfiguration) -> Result<(), CompilationError>;
}

