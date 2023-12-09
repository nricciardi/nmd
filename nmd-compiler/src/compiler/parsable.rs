pub mod codex;
pub mod parsing_configuration;

use std::sync::Arc;

pub use self::codex::parsing_rule::parsing_result::ParsingError;
pub use parsing_configuration::ParsingConfiguration;

pub trait ParallelParsable {
    fn parallel_parse(&mut self, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError>;
}

pub trait SerialParsable {
    fn serial_parse(&mut self, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError>;
}

pub trait Parsable {
    fn parse(&mut self, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError>;
}
