pub mod locatable;

use std::str::FromStr;
use thiserror::Error;
use super::{repository::{Repository, RepositoryError}, compilable::Compilable};
pub use locatable::Locatable;


#[derive(Debug)]
pub enum Location {
    Url(String),
    Path(String)
}

#[derive(Error, Debug)]
pub enum LocationError {
    #[error(transparent)]
    Load(#[from] RepositoryError),

    #[error("location cannot be created: {0}")]
    Creation(String)
}

impl Location {
    pub fn to_string(&self) -> &String {

        match self {
            Self::Url(url) => url,
            Self::Path(path) => path
        }
    }
}

impl Location {
    pub fn load(&self) -> Result<Box<dyn Compilable>, LocationError> {

        Ok(Box::new(Repository::load(self)?))
    }
}

impl FromStr for Location {
    type Err = LocationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if s.trim().eq("") {
            return Err(LocationError::Creation("location cannot be an empty string".to_string()));
        }

        // TODO: improve with match url/path + validation
        Ok(Location::Path(s.to_string()))
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str() {

        let path = ".";

        let repository_location = Location::from_str(path);
    
        match repository_location {
            Ok(location) => assert_eq!(location.to_string(), path),
            Err(e) => panic!("'{}' during location generation from str of path: '{}'", e, path)
        }
    }
}