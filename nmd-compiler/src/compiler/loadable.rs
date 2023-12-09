use thiserror::Error;

use super::resource::{Resource, ResourceError};


#[derive(Error, Debug)]
pub enum LoadError {
    #[error(transparent)]
    ResourceError(#[from] ResourceError)
}

pub trait Loadable {

    fn load(resource: Resource) -> Result<Box<Self>, LoadError>;
}