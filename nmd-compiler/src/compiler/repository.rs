mod dossier;

use thiserror::Error;

pub use self::dossier::Dossier;

use super::{parsable::Parsable, location::{Locatable, Location}, compilable::Compilable};

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

impl Parsable for Repository {
    fn parse(&self) {
        
    }
}

impl Locatable for Repository {
    fn location(&self) -> &Location {
        &self.location
    }
}

impl Compilable for Repository {
    
}

impl Repository {
    pub fn load(location: &Location) -> Result<Self, RepositoryError> {
        todo!("load repository fn missed")
    }
}