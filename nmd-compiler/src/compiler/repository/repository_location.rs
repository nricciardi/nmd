use std::str::FromStr;

pub use super::dossier::Dossier;

use crate::compiler::location::{Location, Locatable};

#[derive(Debug)]
pub struct RepositoryLocation {
    location: Location
}

impl RepositoryLocation {
    
    fn new(location: Location) -> Self {
        RepositoryLocation { location }
    }
}

impl Locatable for RepositoryLocation {

    fn location(&self) -> &Location {
        &self.location
    }
}

impl FromStr for RepositoryLocation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if s.trim().eq("") {
            return Err("location cannot be an empty string");
        }

        // TODO: improve with match url/path + validation
        Ok(RepositoryLocation { location: Location::Path(s.to_string()) })
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_str() {

        let path = ".";

        let repository_location = RepositoryLocation::from_str(path);
    
        match repository_location {
            Ok(repository_location) => assert_eq!(repository_location.location().to_string(), path),
            Err(e) => panic!("'{}' during location generation from str of path: '{}'", e, path)
        }
    }
}
