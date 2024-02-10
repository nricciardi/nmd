use std::sync::Arc;

use crate::compiler::parsable::{codex::Modifier, ParsingConfiguration};

use super::{parsing_outcome::{ParsingError, ParsingOutcome}, ParsingRule};


pub struct HtmlListRule {

}

impl ParsingRule for HtmlListRule {
    fn modifier(&self) -> &Modifier {
        todo!()
    }

    fn parse(&self, content: &str, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError> {
        todo!()
    }
}