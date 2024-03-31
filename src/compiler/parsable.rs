pub mod codex;
pub mod parsing_configuration;

use std::sync::Arc;

use self::codex::{parsing_rule::parsing_outcome::ParsingOutcome, Codex};
pub use self::codex::parsing_rule::parsing_outcome::ParsingError;
pub use parsing_configuration::ParsingConfiguration;


pub trait Parsable {

    // TODO: return ParsingOutcome???
    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError>;

}
