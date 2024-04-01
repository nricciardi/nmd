use std::{io, sync::Arc};

use thiserror::Error;

use crate::resource::ResourceError;

use super::parsable::codex::Codex;




// pub trait Loadable<T> {

//     fn load(codex: Arc<Codex>, resource: &T) -> Result<Box<Self>, LoadError>;
// }