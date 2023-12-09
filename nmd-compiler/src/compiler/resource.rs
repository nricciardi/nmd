mod locatable;          // TODO: future use
mod location;           // TODO: future use

use std::{path::PathBuf, str::FromStr, fs::{File, self}};
use thiserror::Error;


#[derive(Error, Debug)]
pub enum ResourceError {

    #[error("resource '{0}' not found")]
    ResourceNotFound(String),

/*    #[error(transparent)]
     DossierLoadFailed(#[from] DossierError),

    #[error(transparent)]
    DocumentLoadFailed(#[from] DocumentError),
 */
    #[error("resource is invalid")]
    InvalidResource,

    #[error("resource '{0}' is invalid because: {1}")]
    InvalidResourceVerbose(String, String),

    #[error("resource cannot be created: {0}")]
    Creation(String),

    #[error("resource '{0}' cannot be read")]
    ReadError(String),
}

#[derive(Debug)]
pub struct Resource {
    name: String, 
    location: PathBuf       // TODO: migrate to structured type to handle URL
}

impl FromStr for Resource {
    type Err = ResourceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if s.is_empty() {
            return Err(ResourceError::Creation("resource cannot be an empty string".to_string()));
        }

        Ok(Resource::new(PathBuf::from_str(s).unwrap())?)
    }
}


impl ToString for Resource {
    fn to_string(&self) -> String {
        self.location().to_string_lossy().to_string()
    }
}

impl Resource {

    pub fn location(&self) -> &PathBuf {
        &self.location
    } 

    pub fn new(location: PathBuf) -> Result<Self, ResourceError> {
        match location.file_name() {
            Some(name) => Ok(Self {
                name: name.to_string_lossy().to_string(),
                location
            }),
            None => Err(ResourceError::InvalidResource)
        }

        
    }

    pub fn content(&self) -> Result<String, ResourceError> {

        match fs::read_to_string(self.location.clone()) {           // TODO: remove clone
            Ok(content) => Ok(content),
            Err(_) => Err(ResourceError::ReadError(self.to_string()))
        }
    } 

    pub fn name(&self) -> &String {
        &self.name        
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn from_str() {

        let path = ".";

        let resource = Resource::from_str(path);
    
        match resource {
            Ok(location) => assert_eq!(location.to_string(), path),
            Err(e) => panic!("'{}' during location generation from str of path: '{}'", e, path)
        }
    }
}
