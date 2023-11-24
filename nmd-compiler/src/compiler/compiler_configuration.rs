use std::str::FromStr;
use std::string::ParseError;
use thiserror::Error;
use super::location;
use super::location::Locatable;
use super::supported_format::SupportedFormat;
use super::supported_format::SupportedFormatError;
use super::repository::RepositoryLocation;


#[derive(Error, Debug)]
pub enum CompilerConfigurationError {
    #[error("configuration error: {0}")]
    SupportedFormatError(SupportedFormatError),

    #[error("parse error: {0}")]
    ParseError(String)
}

impl From<SupportedFormatError> for CompilerConfigurationError {
    fn from(value: SupportedFormatError) -> Self {
        CompilerConfigurationError::SupportedFormatError(value)
    }
}

pub struct CompilerConfiguration {
    location: Box<dyn Locatable>,
    format: SupportedFormat
}

impl CompilerConfiguration {
    pub fn new(location: &str, format: &str) -> Result<Self, CompilerConfigurationError> {

        let location = match RepositoryLocation::from_str(location) {           // TODO: dynamic location, not only repository, but also only one dossier or document
            Ok(l) => l,
            Err(e) => return Err(CompilerConfigurationError::ParseError(e.to_string()))
        };

        Ok(CompilerConfiguration {
            location: Box::new(location),
            format: SupportedFormat::from_str(format)?
        })
    }
}