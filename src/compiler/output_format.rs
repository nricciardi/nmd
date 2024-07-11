use std::str::FromStr;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum OutputFormatError {
    #[error("unsupported format: {0}")]
    Unsupported(String)
}

/// Set of supported formats
#[derive(PartialEq, Debug, Default, Clone)]
pub enum OutputFormat {
    #[default]
    Html
}

impl OutputFormat {
    pub fn get_extension(&self) -> String {
        match self {
            OutputFormat::Html => String::from("html"),
        }
    } 
}

impl FromStr for OutputFormat {

    type Err = OutputFormatError;

    fn from_str(format: &str) -> Result<Self, Self::Err> {
        match format.to_lowercase().as_str() {
            "html" => Ok(Self::Html),
            
            _ => Err(OutputFormatError::Unsupported(String::from(format))),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn html_support() {

        match OutputFormat::from_str("html") {
            Ok(format) => assert_eq!(format, OutputFormat::Html),
            Err(err) => panic!("{}", err)
        }
    }

    #[test]
    fn unsupported_format() {
        assert!(OutputFormat::from_str("htm").is_err())
    }
}