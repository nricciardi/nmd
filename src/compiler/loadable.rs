use std::{io, sync::Arc};

use thiserror::Error;

use crate::resource::ResourceError;

use super::parsable::codex::Codex;


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

    fn load(codex: Arc<Codex>, resource: &T) -> Result<Box<Self>, LoadError>;
}