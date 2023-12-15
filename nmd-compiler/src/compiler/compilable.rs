pub mod compilation_configuration;

use std::sync::Arc;

use self::compilation_configuration::CompilationConfiguration;

use super::{parsable::Parsable, loadable::{Loadable, LoadError}, dumpable::Dumpable};
use anyhow::Result;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum CompilationError {

    #[error(transparent)]
    LoadError(#[from] LoadError),

    #[error("unknown error")]
    Unknown(String)
}

pub trait Compilable: Loadable + Parsable + Dumpable {
    fn compile(&self, compilation_configuration: Arc<CompilationConfiguration>) -> Result<(), CompilationError>;
}

