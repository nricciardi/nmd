pub mod disk_resource;
pub mod cached_disk_resource;
pub mod image_resource;
pub mod remote_resource;
pub mod dynamic_resource;
pub mod resource_reference;
pub mod text_reference;
pub mod table;

use std::{str::FromStr, io::{self}};
use thiserror::Error;


#[derive(Error, Debug)]
pub enum ResourceError {

    #[error("resource '{0}' not found")]
    ResourceNotFound(String),

    #[error("resource is invalid")]
    InvalidResource,

    #[error("resource is invalid because: {0}")]
    InvalidResourceVerbose(String),

    #[error("resource cannot be created: {0}")]
    Creation(String),

    #[error("resource '{0}' cannot be read")]
    ReadError(String),

    #[error(transparent)]
    IoError(#[from] io::Error),
    
    #[error("elaboration error: {0}")]
    ElaborationError(String),
}

impl Clone for ResourceError {
    fn clone(&self) -> Self {
        match self {
            Self::IoError(e) => Self::ElaborationError(e.to_string()),
            other => other.clone()
        }
    }
}


/// General physical or virtual resource
pub trait Resource: FromStr {

    type LocationType;

    /// write resource content
    fn write(&mut self, content: &str) -> Result<(), ResourceError>;

    /// erase content resource
    fn erase(&mut self) -> Result<(), ResourceError>;

    /// append resource content
    fn append(&mut self, content: &str) -> Result<(), ResourceError>;

    /// read resource content
    fn read(&self) -> Result<String, ResourceError>;

    /// return resource content
    fn content(&self) -> Result<String, ResourceError> {
        self.read()        
    }

    /// return resource name
    fn name(&self) -> &String;

    /// return embedded location type (e.g. PathBuf for files)
    fn location(&self) -> &Self::LocationType;
}