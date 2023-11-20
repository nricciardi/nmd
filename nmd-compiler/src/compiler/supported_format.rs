use std::str::FromStr;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum SupportedFormatError {
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String)
}

#[derive(PartialEq, Debug)]
pub enum SupportedFormat {
    Html
}

impl FromStr for SupportedFormat {

    type Err = SupportedFormatError;

    fn from_str(format: &str) -> Result<Self, Self::Err> {
        match format.to_lowercase().as_str() {
            "html" => Ok(SupportedFormat::Html),
            _ => Err(SupportedFormatError::UnsupportedFormat(String::from(format))),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn html_support() {

        match SupportedFormat::from_str("html") {
            Ok(format) => assert_eq!(format, SupportedFormat::Html),
            Err(err) => panic!("{}", err)
        }
    }

    #[test]
    fn unsupported_format() {
        assert!(SupportedFormat::from_str("htm").is_err())
    }
}