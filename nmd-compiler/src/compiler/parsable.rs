pub mod codex;
pub mod parsing_result;
pub mod parsing_configuration;

pub use self::parsing_result::ParsingResult;
pub use self::parsing_configuration::ParsingConfiguration;


pub trait Parsable {
    fn parse(&self, parsing_configuration: &ParsingConfiguration) -> ParsingResult;
}
