use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("pattern provided '{0}' is invalid")]
    InvalidPattern(&'static str),

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

    pub fn parsed_content(self: Self) -> String {
        self.parsed_content
    }
}