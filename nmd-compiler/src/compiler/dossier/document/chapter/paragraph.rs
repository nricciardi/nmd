use std::{sync::Arc, fmt::Display};

use regex::Regex;
use thiserror::Error;

use crate::compiler::parsable::{codex::{parsing_rule::parsing_result::ParsingError, Codex}, Parsable};
use crate::compiler::parsable::parsing_configuration::ParsingConfiguration;

#[derive(Error, Debug)]
pub enum ParagraphError {
    #[error("creation error")]
    Creation
}

pub struct Paragraph {
    content: String
}

impl Clone for Paragraph {
    fn clone(&self) -> Self {
        Self { content: self.content.clone() }
    }
}

impl Parsable for Paragraph {
    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError> {

        self.content = codex.parse(&self.content, Arc::clone(&parsing_configuration))?.parsed_content();

        Ok(())
    }
}

impl Paragraph {
    pub fn from_str(content: &str) -> Option<Vec<Self>> {

        if content.is_empty() {
            return Option::None;
        }

        let regex = Regex::new("\n\n").unwrap();

        Option::Some(regex.split(content).map(|splitted_content| {
            Self {
                content: splitted_content.to_string()
            }
        }).collect())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn from_str() {
        let content = r#"
this is the first paragraph.
Paragraph 1 continued.

Paragraph 2.

Paragraph 3.        
"#.trim();

        let paragraphs = Paragraph::from_str(content).unwrap();

        assert_eq!(paragraphs.len(), 3)
    }
}

impl Display for Paragraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}