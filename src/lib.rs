pub mod compiler;
pub mod resource;
pub mod generator;
pub mod dossier_manager;
mod utility;
mod config;
pub mod cli;

use std::{path::PathBuf, str::FromStr};

use clap::{Arg, ArgAction, Command};
use compiler::{output_format::OutputFormatError, CompilationError};
pub use compiler::Compiler;
use dossier_manager::{dossier_manager_configuration::{self, DossierManagerConfiguration}, DossierManager, DossierManagerError};
use generator::{generator_configuration::GeneratorConfiguration, Generator};
use log::{LevelFilter, ParseLevelError};
use resource::ResourceError;
use thiserror::Error;
use simple_logger::SimpleLogger;

use crate::compiler::{compilation_configuration::CompilationConfiguration, output_format::OutputFormat};

pub const VERSION: &str = "0.21.0";
