pub mod codex;
pub mod parsing_configuration;

pub use self::codex::parsing_rule::parsing_result::ParsingError;
pub use parsing_configuration::ParsingConfiguration;


pub trait Parsable {
    fn parse(&mut self, parsing_configuration: &ParsingConfiguration) -> Result<(), ParsingError>;
}
