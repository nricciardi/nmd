use std::io;

use thiserror::Error;

use super::resource::ResourceError;


#[derive(Error, Debug)]
pub enum LoadError {
    #[error(transparent)]
    ResourceError(#[from] ResourceError),

    #[error("elaboration error: {0}")]
    ElaborationError(String),
    
    #[error(transparent)]
    IoError(#[from] io::Error)
}

pub trait Loadable<T> {

    fn load(resource: &T) -> Result<Box<Self>, LoadError>;
}