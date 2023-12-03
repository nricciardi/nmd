pub mod locatable;

use std::{str::FromStr, ffi::OsStr, fs};
use thiserror::Error;
use super::{dossier::{Dossier, DossierError, Document, DocumentError}, compilable::Compilable};
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

    #[error("resource '{0}' not found")]
    ResourceNotFound(String),

    #[error("resource '{0}' unexpected")]
    ResourceUnexpected(String),

    #[error(transparent)]
    DossierLoadFailed(#[from] DossierError),

    #[error(transparent)]
    DocumentLoadFailed(#[from] DocumentError),

    #[error("resource '{0}' is invalid")]
    InvalidResource(String),

    #[error("location cannot be created: {0}")]
    Creation(String)
}


impl Location {
    pub fn load(&self) -> Result<Box<dyn Compilable>, LocationError> {

        match self {
            Self::LocalPath(path) => {

                if !path.exists() {
                    return Err(LocationError::ResourceNotFound(path.to_string_lossy().to_string()));
                }

                if path.is_file() {
                    return Ok(Box::new(Document::load(self)?));
                
                } else if path.is_dir() {

                    return Ok(Box::new(Dossier::load(self)?));
                }
                
                Err(LocationError::ResourceUnexpected(path.to_string_lossy().to_string()))
                
            },
            Self::Url(_url) => todo!()
        }
    }

    pub fn resource_name(&self) -> &OsStr {
        match self {
            Self::LocalPath(path) => {
                path.file_name().expect("invalid resource")
            },

            Self::Url(_url) => todo!()
        }
    }

    pub fn subresources(&self) -> Option<Vec<Location>> {
        match self {
            Self::LocalPath(path) => {
                if !path.is_dir() {
                    return None
                }

                let entries = fs::read_dir(path).expect("invalid resource");

                let mut subresource: Vec<Location> = Vec::new();

                for entry in entries {
                    if let Ok(entry) = entry {
                        let file_path = entry.path();

                        subresource.push(Location::LocalPath(file_path));
                    }
                }

                Some(subresource)
            },

            Self::Url(_url) => todo!()
        }
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

impl From<PathBuf> for Location {
    fn from(value: PathBuf) -> Self {
        Location::LocalPath(value)
    }
}

impl Location {

    pub fn to_string(&self) -> String {
        match self {
            Self::Url(url) => url.to_string(),
            Self::LocalPath(path) => path.clone().to_string_lossy().to_string()
        }
    }
}


#[cfg(test)]
mod test {
    use std::env;

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

    #[test]
    fn load() {

        let project_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let dossier_name = "nmd-test-dossier-1";
        let project_directory = project_directory.join("test-resources").join(dossier_name);

        assert!(project_directory.is_dir());

        let location = Location::from(project_directory);

        assert_eq!(location.resource_name().to_str().unwrap(), dossier_name);

        let physical_subresources = vec!["d1.nmd", "d2.nmd"];
        let subresources = location.subresources().unwrap();

        let subresources: Vec<String> = subresources.iter().map(|sr| {
            sr.resource_name().to_string_lossy().to_string()
        }).collect();

        for psr in physical_subresources {
            assert!(subresources.contains(&psr.to_string()))
        }
    }
}