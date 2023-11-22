use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("unknown error occurs")]
    Unknown
}


pub struct ParsingResultBody {

}

pub type ParsingResult = Result<ParsingResultBody, ParsingError>;
