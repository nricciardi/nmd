use std::error;
use std::str::FromStr;
use thiserror::Error;
use super::supported_format::SupportedFormat;
use super::supported_format::SupportedFormatError;


#[derive(Error, Debug)]
pub enum CompilerConfigurationError {
    #[error("configuration error: {0}")]
    SupportedFormatError(SupportedFormatError)
}

impl From<SupportedFormatError> for CompilerConfigurationError {
    fn from(value: SupportedFormatError) -> Self {
        CompilerConfigurationError::SupportedFormatError(value)
    }
}

pub struct CompilerConfiguration {
    format: SupportedFormat
}

impl CompilerConfiguration {
    pub fn new(format: &str) -> Result<Self, CompilerConfigurationError> {
        Ok(CompilerConfiguration {
            format: SupportedFormat::from_str(format)?
        })
    }
}