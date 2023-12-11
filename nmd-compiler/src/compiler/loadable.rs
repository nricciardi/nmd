use thiserror::Error;

use super::resource::{Resource, ResourceError};


#[derive(Error, Debug)]
pub enum LoadError {
    #[error(transparent)]
    ResourceError(#[from] ResourceError)
}

pub trait Loadable {

    type Type;

    fn load(resource: Self::Type) -> Result<Box<Self>, LoadError>;
}