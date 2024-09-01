use nmd_core::{assembler::AssemblerError, compiler::compilation_error::CompilationError, dumpable::DumpError, loader::LoadError};
use thiserror::Error;
use tokio::task::JoinError;

use crate::{preview::PreviewError, watcher::WatcherError};

#[derive(Error, Debug)]
pub enum BuilderError {

    #[error("unknown error")]
    Unknown(String),

    #[error(transparent)]
    LoadError(#[from] LoadError),

    #[error(transparent)]
    CompilationError(#[from] CompilationError),

    #[error(transparent)]
    AssemblerError(#[from] AssemblerError),

    #[error(transparent)]
    DumpError(#[from] DumpError),

    #[error(transparent)]
    PreviewError(#[from] PreviewError),

    #[error(transparent)]
    WatcherError(#[from] WatcherError),

    #[error(transparent)]
    JoinError(#[from] JoinError),
}