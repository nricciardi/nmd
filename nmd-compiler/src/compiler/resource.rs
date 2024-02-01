pub mod disk_resource;
pub mod cached_disk_resource;
pub mod image;
pub mod remote_resource;

use std::{path::PathBuf, str::FromStr, fs::{File, self, OpenOptions}, io::{self, Write}};
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
    IoError(#[from] io::Error)
}

pub trait Resource: FromStr {

    type LocationType;

    fn write(&mut self, content: &str) -> Result<(), ResourceError>;

    fn append(&mut self, content: &str) -> Result<(), ResourceError>;

    fn read(&self) -> Result<String, ResourceError>;

    fn content(&self) -> Result<String, ResourceError> {
        self.read()        
    }

    fn name(&self) -> &String;

    fn location(&self) -> &Self::LocationType;
}