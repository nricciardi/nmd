use std::{sync::Arc, fmt::Display};

use regex::Regex;
use thiserror::Error;

use crate::compiler::parsable::{codex::{parsing_rule::parsing_outcome::ParsingError, Codex}, Parsable};
use crate::compiler::parsable::parsing_configuration::ParsingConfiguration;

#[derive(Error, Debug)]
pub enum ParagraphError {
    #[error("creation error")]
    Creation,

    #[error("empty content")]
    Empty
}

pub struct Paragraph {
    content: String
}

impl Paragraph {
    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn contains_only_newlines(&self) -> bool {
        self.content.chars().all(|c| c == '\n')
    }
}

impl Clone for Paragraph {
    fn clone(&self) -> Self {
        Self { content: self.content.clone() }
    }
}

impl Parsable for Paragraph {
    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError> {

        let parsing_outcome = codex.parse_paragraph(self, Arc::clone(&parsing_configuration))?;

        self.content = String::from(parsing_outcome.parsed_content());

        Ok(())
    }
}

impl From<String> for Paragraph {
    fn from(content: String) -> Self {
        Self {
            content
        }
    }
}

impl Paragraph {

    pub fn str_to_paragraphs(s: &str) -> Result<Vec<Self>, ParagraphError> {
        if s.is_empty() {
            return Err(ParagraphError::Empty);
        }

        let regex = Regex::new(r#"\n{2,}(?!```)"#).unwrap();

        Ok(regex.split(s).map(|splitted_content| {
            Self::from(splitted_content.to_string())
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

        let paragraphs = Paragraph::str_to_paragraphs(content).unwrap();

        assert_eq!(paragraphs.len(), 3)
    }
}

impl Display for Paragraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}