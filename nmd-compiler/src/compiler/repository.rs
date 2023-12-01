mod dossier;

use thiserror::Error;

pub use self::dossier::Dossier;
use super::parsable::{Parsable, ParsingConfiguration, ParsingResult};
use super::{location::{Locatable, Location}, compilable::{Compilable, CompilationConfiguration}};

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("repository loading failed: '{0}'")]
    Load(&'static str)
}

pub struct Repository {
    location: Location,
    name: String,
    dossiers: Option<Vec<Dossier>>
}

impl Locatable for Repository {
    fn location(&self) -> &Location {
        &self.location
    }
}

impl Parsable for Repository {
    fn parse(&self, parsing_configuration: &ParsingConfiguration) -> ParsingResult {
        todo!()
    }
}

impl Compilable for Repository {
    fn compile(&self, compilation_configuration: CompilationConfiguration) -> anyhow::Result<()> {
        todo!()
    }
}

impl Repository {
    pub fn load(location: &Location) -> Result<Self, RepositoryError> {
        todo!("load repository fn missed")
    }
}