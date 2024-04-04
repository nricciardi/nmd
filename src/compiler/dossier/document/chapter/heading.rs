use std::{num::ParseIntError, str::FromStr, sync::Arc};

use crate::compiler::{codex::Codex, parser::{parsable::Parsable, parsed_content_accessor::ParsedContentAccessor, parsing_rule::{parsing_configuration::ParsingConfiguration, parsing_error::ParsingError, parsing_outcome::ParsingOutcome}, Parser}};

use super::chapter_builder::ChapterBuilderError;


#[derive(Debug, Clone)]
pub enum HeadingLevel {
    PrecedentePlusOne,
    PrecedenteMinusOne,
    Numerical(u32)
}

impl FromStr for HeadingLevel {
    type Err = ParseIntError;     // TODO

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq("+") {
            return Ok(Self::PrecedentePlusOne)
        }

        if s.eq("-") {
            return Ok(Self::PrecedenteMinusOne)
        }

        match s.parse::<u32>() {
            Ok(n) => Ok(Self::Numerical(n)),
            Err(e) => Err(e)
        }
    }
}


#[derive(Debug, Clone)]
pub struct Heading {
    // level: HeadingLevel,
    // title: String,

    raw_content: String,
    parsed_content: Option<ParsingOutcome>
}

impl Heading {
    pub fn new(raw_content: String) -> Self {
        Self {
            raw_content,
            parsed_content: None
        }
    }

    // pub fn level(&self) -> &HeadingLevel {
    //     &self.level
    // }

    // pub fn title(&self) -> &String {
    //     &self.title
    // }
}

impl Parsable for Heading {
    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError> {
        let parsing_outcome = Parser::parse_text(&codex, &self.raw_content, &parsing_configuration)?;

        self.parsed_content = Some(parsing_outcome.parsed_content());

        Ok(())
    }
}


impl ParsedContentAccessor for Heading {
    fn parsed_content(&self) -> &Option<ParsingOutcome> {
        &self.parsed_content
    }
}
