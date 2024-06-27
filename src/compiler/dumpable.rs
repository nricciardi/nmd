use thiserror::Error;

use crate::resource::ResourceError;


#[derive(Debug, Error)]
pub enum DumpError {
    #[error(transparent)]
    ResourceError(#[from] ResourceError)
}


/// Dump trait. Dump is the operation which permits to save save a resource
pub trait Dumpable {

    fn dump(&mut self) -> Result<(), DumpError>;
} 