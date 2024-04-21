pub mod replacement_rule;

pub mod html_image_rule;
pub mod html_list_rule;
pub mod html_extended_block_quote_rule;

use std::{fmt::Debug, sync::Arc};
use regex::Regex;

use super::{parsing_configuration::ParsingConfiguration, parsing_error::ParsingError, parsing_outcome::ParsingOutcome};


pub trait ParsingRule: Send + Sync + Debug {

    fn searching_pattern(&self) -> &String;

    // TODO?: fn replacing_pattern(&self) -> &R;         // Replacer

    fn is_match(&self, content: &str) -> bool {

        let pattern = self.searching_pattern();

        let regex = Regex::new(&pattern).unwrap();

        regex.is_match(content)
    }

    fn parse(&self, content: &str, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError>;

}


// #[cfg(test)]
// mod test {
//     use crate::compiler::codex::Modifier;

//     #[test]
//     fn is_heading() {
//         let content = "#6 title 6";

//         assert!(Modifier::heading_level(content).is_some());

//         let content = "### title 3";

//         assert!(Modifier::heading_level(content).is_some());

//         let content = "text";

//         assert!(Modifier::heading_level(content).is_none())
//     }
// }