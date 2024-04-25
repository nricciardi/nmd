use thiserror::Error;

use crate::resource::ResourceError;


#[derive(Debug, Error)]
pub enum DumpError {
    #[error(transparent)]
    ResourceError(#[from] ResourceError)
}


pub trait Dumpable {

    fn dump(&mut self) -> Result<(), DumpError>;
} 