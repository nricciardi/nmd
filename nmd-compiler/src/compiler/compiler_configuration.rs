use std::str::FromStr;
use thiserror::Error;
use super::compilable::compilation_configuration::CompilationConfiguration;
use super::location::Location;
use super::location::LocationError;
use super::supported_format::SupportedFormatError;


#[derive(Error, Debug)]
pub enum CompilerConfigurationError {
    #[error(transparent)]
    SupportedFormatError(#[from] SupportedFormatError),

    #[error("parse error: {0}")]
    ParseError(String),

    #[error(transparent)]
    LocationError(#[from] LocationError)
}

/// Configuration to build a Compiler
pub struct CompilerConfiguration {
    compilation_configuration: CompilationConfiguration
}

impl CompilerConfiguration {
    // Create new configuration from str value
    pub fn new(location: &str, format: &str) -> Result<Self, CompilerConfigurationError> {

        Ok(CompilerConfiguration {
            compilation_configuration: {
                CompilationConfiguration::default()
                // TODO
            }         
        })
    }
    
    pub fn compilation_configuration(&self) -> &CompilationConfiguration {
        &self.compilation_configuration
    }
}


// TODO: CompilerConfigurationBuilder 