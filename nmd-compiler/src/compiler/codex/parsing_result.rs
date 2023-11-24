use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("pattern provided '{0}' is invalid")]
    InvalidPattern(&'static str),

    #[error("unknown error occurs")]
    Unknown
}


pub struct ParsingResultBody {
    parsed_content: String
}

impl ParsingResultBody {
    pub fn new(parsed_content: String) -> ParsingResultBody {
        ParsingResultBody{
            parsed_content
        }
    }

    pub fn parsed_content(self: &Self) -> String {
        self.parsed_content.clone()        // TODO: remove clone
    }
}



pub type ParsingResult = Result<ParsingResultBody, ParsingError>;

