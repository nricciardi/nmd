pub mod codex;


use self::codex::parsing_rule::{ParsingConfiguration, parsing_result::ParsingError};


pub trait Parsable {
    fn parse(&mut self, parsing_configuration: &ParsingConfiguration) -> Result<(), ParsingError>;
}
