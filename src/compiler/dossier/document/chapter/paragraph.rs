use std::{sync::Arc, fmt::Display};

use regex::Regex;
use thiserror::Error;

use crate::compiler::parsable::{codex::{modifier::{paragraph_modifier::ParagraphModifier, ModifierIdentifier}, parsing_rule::parsing_outcome::{ParsingError, ParsingOutcome}, Codex}, Parsable};
use crate::compiler::parsable::parsing_configuration::ParsingConfiguration;

#[derive(Error, Debug)]
pub enum ParagraphError {
    #[error("creation error")]
    Creation,

    #[error("empty content")]
    Empty
}

#[derive(Debug, Clone)]
pub struct Paragraph {
    content: String,
    paragraph_type: ModifierIdentifier,
}

impl Paragraph {

    pub fn new(content: String, paragraph_type: ModifierIdentifier) -> Self {
        Self {
            content,
            paragraph_type
        }
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn contains_only_newlines(&self) -> bool {
        self.content.chars().all(|c| c == '\n')
    }

    pub fn paragraph_type(&self) -> &ModifierIdentifier {
        &self.paragraph_type
    }
}


impl Parsable for Paragraph {
    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError> {

        codex.parse_paragraph(self, Arc::clone(&parsing_configuration))
    }
}

impl Display for Paragraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

// impl Paragraph {

//     pub fn str_to_paragraphs(s: &str) -> Result<Vec<Self>, ParagraphError> {
//         if s.is_empty() {
//             return Err(ParagraphError::Empty);
//         }

//         let regex = Regex::new(r#"\n{2,}(?!```)"#).unwrap();

//         Ok(regex.split(s).map(|splitted_content| {
//             Self::from(splitted_content.to_string())
//         }).collect())
//     }
// }

#[cfg(test)]
mod test {

    use super::*;

//     #[test]
//     fn from_str() {
//         let content = r#"
// this is the first paragraph.
// Paragraph 1 continued.

// Paragraph 2.

// Paragraph 3.        
// "#.trim();

//         let paragraphs = Paragraph::str_to_paragraphs(content).unwrap();

//         assert_eq!(paragraphs.len(), 3)
//     }
}