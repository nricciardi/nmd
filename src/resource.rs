pub mod disk_resource;
pub mod cached_disk_resource;
pub mod image_resource;
pub mod remote_resource;

use std::{path::PathBuf, str::FromStr, fs::{File, self, OpenOptions}, io::{self, Write}};
use ::image::ImageError;
use serde_json::error;
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

    #[error(transparent)]
    ImageError(#[from] ImageError),
    
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


pub trait Resource: FromStr {

    type LocationType;

    fn write(&mut self, content: &str) -> Result<(), ResourceError>;

    fn erase(&mut self) -> Result<(), ResourceError>;

    fn append(&mut self, content: &str) -> Result<(), ResourceError>;

    fn read(&self) -> Result<String, ResourceError>;

    fn content(&self) -> Result<String, ResourceError> {
        self.read()        
    }

    fn name(&self) -> &String;

    fn location(&self) -> &Self::LocationType;
}