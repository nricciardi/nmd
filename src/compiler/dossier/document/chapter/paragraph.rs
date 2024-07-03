use std::{fmt::Display, sync::{Arc, RwLock}};

use regex::Regex;
use thiserror::Error;

use crate::compiler::{codex::{modifier::ModifierIdentifier, Codex}, output_format::OutputFormat, parsable::{parsed_content_accessor::ParsedContentAccessor, Parsable}, parser::Parser, parsing::{parsing_configuration::{parsing_configuration_overlay::ParsingConfigurationOverLay, ParsingConfiguration}, parsing_error::ParsingError, parsing_metadata::ParsingMetadata, parsing_outcome::ParsingOutcome}};


#[derive(Error, Debug)]
pub enum ParagraphError {
    #[error("creation error")]
    Creation,

    #[error("empty content")]
    Empty
}

pub type ParagraphType = ModifierIdentifier;

#[derive(Debug, Clone)]
pub struct Paragraph {
    content: String,
    parsed_content: Option<ParsingOutcome>,
    paragraph_type: ParagraphType,
}

impl Paragraph {

    pub fn new(content: String, paragraph_type: ParagraphType) -> Self {
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
        self.content.chars().all(|c| c == '\n' || c == '\r')
    }

    pub fn paragraph_type(&self) -> &ParagraphType {
        &self.paragraph_type
    }
}

impl Parsable for Paragraph {
    fn standard_parse(&mut self, format: &OutputFormat, codex: Arc<Codex>, parsing_configuration: Arc<RwLock<ParsingConfiguration>>, parsing_configuration_overlay: Arc<Option<ParsingConfigurationOverLay>>) -> Result<(), ParsingError> {

        let parsing_outcome = Parser::parse_paragraph(&codex, self, Arc::clone(&parsing_configuration), parsing_configuration_overlay)?;

        self.parsed_content = Some(parsing_outcome);

        Ok(())
    }
}

impl ParsedContentAccessor for Paragraph {
    fn parsed_content(&self) -> &Option<ParsingOutcome> {
        &self.parsed_content
    }
}

impl Display for Paragraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}
