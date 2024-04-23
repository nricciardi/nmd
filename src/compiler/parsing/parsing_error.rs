use clap::error;
use thiserror::Error;

use crate::{compiler::codex::reference::ReferenceError, resource::ResourceError};

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("pattern provided '{0}' is invalid")]
    InvalidPattern(String),

    #[error("'{0}' is an invalid source")]
    InvalidSource(String),

    #[error("failed during parsing elaboration")]
    ElaborationError,

    #[error("'{0}' is an invalid parameter")]
    InvalidParameter(String),

    #[error(transparent)]
    ReferenceError(#[from] ReferenceError),

    #[error("unknown error occurs")]
    Unknown,
}