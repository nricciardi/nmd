use std::{num::ParseIntError, str::FromStr, sync::Arc};

use crate::compiler::{codex::Codex, parser::{parsable::Parsable, parsed_content_accessor::ParsedContentAccessor, parsing_rule::{parsing_configuration::ParsingConfiguration, parsing_error::ParsingError, parsing_outcome::ParsingOutcome}, Parser}};

use super::chapter_builder::ChapterBuilderError;


// #[derive(Debug, Clone)]
// pub enum HeadingLevel {
//     PreviousPlusOne,
//     PreviousMinusOne,
//     Numerical(u32)
// }

// impl FromStr for HeadingLevel {
//     type Err = ParseIntError;     // TODO

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         if s.eq("+") {
//             return Ok(Self::PreviousPlusOne)
//         }

//         if s.eq("-") {
//             return Ok(Self::PreviousMinusOne)
//         }

//         match s.parse::<u32>() {
//             Ok(n) => Ok(Self::Numerical(n)),
//             Err(e) => Err(e)
//         }
//     }
// }

pub type HeadingLevel = u32;


#[derive(Debug, Clone)]
pub struct Heading {
    level: HeadingLevel,
    title: String,

    // raw_content: String,
    parsed_content: Option<ParsingOutcome>
}

impl Heading {
    pub fn new(level: HeadingLevel, title: String) -> Self {

        Self {
            level,
            title,
            parsed_content: None
        }
    }

    pub fn level(&self) -> HeadingLevel {
        self.level
    }

    pub fn title(&self) -> &String {
        &self.title
    }
}

impl Parsable for Heading {
    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError> {
        let id = Codex::create_id(&self.title);

        let parsed_title = Parser::parse_text(&codex, &self.title, Arc::clone(&parsing_configuration))?;

        self.parsed_content = Some(ParsingOutcome::new(format!(r#"<h{} class="heading-{}" id="{}">{}</h{}>"#, self.level, self.level, id, parsed_title.parsed_content(), self.level)));

        Ok(())
    }
}


impl ParsedContentAccessor for Heading {
    fn parsed_content(&self) -> &Option<ParsingOutcome> {
        &self.parsed_content
    }
}
