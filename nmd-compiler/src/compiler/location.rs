pub mod locatable;

use std::str::FromStr;
use thiserror::Error;
use super::{repository::{Repository, RepositoryError}, compilable::Compilable};
pub use locatable::Locatable;
use url::Url;
use std::path::PathBuf;


#[derive(Debug)]
pub enum Location {
    Url(Url),
    LocalPath(PathBuf)
}

#[derive(Error, Debug)]
pub enum LocationError {
    #[error(transparent)]
    Load(#[from] RepositoryError),

    #[error("location cannot be created: {0}")]
    Creation(String)
}

impl Location {

    pub fn to_string(&self) -> String {
        match self {
            Self::Url(url) => url.to_string(),
            Self::LocalPath(path) => path.clone().to_string_lossy().to_string()
        }
    }
}

impl Location {
    pub fn load(&self) -> Result<Box<dyn Compilable>, LocationError> {

        todo!();

        // Ok(Box::new(Repository::load(self)?))
    }
}

impl FromStr for Location {
    type Err = LocationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if s.trim().eq("") {
            return Err(LocationError::Creation("location cannot be an empty string".to_string()));
        }

        if let Ok(url) = Url::parse(s) {
            
            return Ok(Location::Url(url));

        } else {
            return Ok(Location::LocalPath(PathBuf::from(s)));
        }
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