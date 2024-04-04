use std::{sync::Arc, fmt::Display};

use regex::Regex;
use thiserror::Error;

use crate::compiler::{codex::{modifier::ModifierIdentifier, Codex}, parser::{parsable::Parsable, parsed_content_accessor::ParsedContentAccessor, parsing_rule::{parsing_configuration::ParsingConfiguration, parsing_error::ParsingError, parsing_outcome::ParsingOutcome}}};


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
    parsed_content: Option<ParsingOutcome>,
    paragraph_type: ModifierIdentifier,
}

impl Paragraph {

    pub fn new(content: String, paragraph_type: ModifierIdentifier) -> Self {
        Self {
            content,
            paragraph_type,
            parsed_content: None
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
    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError> {

        let parsing_outcome = codex.parse_paragraph(self, Arc::clone(&parsing_configuration))?;

        self.parsed_content = Some(parsing_outcome.parsed_content());

        Ok(())
    }
}

impl ParsedContentAccessor for Paragraph {
    fn parsed_content(&self) -> &Option<ParsingOutcome> {
        &self.parsed_content
    }
}


// impl Parsable for Paragraph {
//     fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError> {

//         codex.parse_paragraph(self, Arc::clone(&parsing_configuration))
//     }
// }

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