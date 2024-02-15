use clap::error;
use thiserror::Error;

use crate::resource::ResourceError;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("pattern provided '{0}' is invalid")]
    InvalidPattern(String),

    #[error("'{0}' is an invalid source")]
    InvalidSource(String),

    #[error("unknown error occurs")]
    Unknown
}


pub struct ParsingOutcome {
    parsed_content: String
}

impl ParsingOutcome {
    pub fn new(parsed_content: String) -> Self {
        Self {
            parsed_content
        }
    }

    pub fn parsed_content(&self) -> &String {
        &self.parsed_content
    }
}