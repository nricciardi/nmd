use std::str::FromStr;
use thiserror::Error;
use super::compilable::Compilable;
use super::location::Location;
use super::location::LocationError;
use super::supported_format::SupportedFormat;
use super::supported_format::SupportedFormatError;


#[derive(Error, Debug)]
pub enum CompilerConfigurationError {
    #[error("configuration error: {0}")]
    SupportedFormatError(SupportedFormatError),

    #[error("parse error: {0}")]
    ParseError(String),

    #[error("location error: {0}")]
    LocationError(LocationError)
}

impl From<SupportedFormatError> for CompilerConfigurationError {
    fn from(value: SupportedFormatError) -> Self {
        CompilerConfigurationError::SupportedFormatError(value)
    }
}

pub struct CompilerConfiguration {
    location: Location,
    format: SupportedFormat
}

impl CompilerConfiguration {
    pub fn new(location: &str, format: &str) -> Result<Self, CompilerConfigurationError> {


        // TODO: improve error handling with ? 
        let location = match Location::from_str(location) {           // TODO: dynamic location, not only repository, but also only one dossier or document
            Ok(l) => l,
            Err(e) => return Err(CompilerConfigurationError::LocationError(e))
        };

        Ok(CompilerConfiguration {
            location,
            format: SupportedFormat::from_str(format)?
        })
    }

    pub fn location(&self) -> &Location {
        &self.location
    }
}