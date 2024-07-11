use std::path::PathBuf;

use getset::{CopyGetters, Getters, Setters};
use thiserror::Error;

use crate::resource::ResourceError;


#[derive(Debug, Error)]
pub enum DumpError {
    #[error(transparent)]
    ResourceError(#[from] ResourceError)
}


#[derive(Debug, Clone, Getters, CopyGetters, Setters)]
pub struct DumpConfiguration {
    
    #[getset(get = "pub", set = "pub")]
    output_path: PathBuf,

    #[getset(get_copy = "pub", set = "pub")]
    force_dump: bool,
}

impl DumpConfiguration {
    pub fn new(output_path: PathBuf, force_dump: bool,) -> Self {
        Self {
            output_path,
            force_dump
        }
    }
}


/// Dump trait. Dump is the operation which permits to save a resource
pub trait Dumpable {

    fn dump(&mut self, configuration: &DumpConfiguration) -> Result<(), DumpError>;
} 