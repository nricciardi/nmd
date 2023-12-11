pub mod codex;
pub mod parsing_configuration;

use std::sync::Arc;

use self::codex::Codex;
pub use self::codex::parsing_rule::parsing_result::ParsingError;
pub use parsing_configuration::ParsingConfiguration;


pub trait Parsable {
    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError>;
}
