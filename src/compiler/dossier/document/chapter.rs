pub mod paragraph;
pub mod heading;
pub mod chapter_builder;

use std::fmt::Display;
use std::sync::Arc;

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use self::heading::Heading;
pub use self::paragraph::Paragraph;
use crate::compiler::parsable::codex::parsing_rule::parsing_outcome::{self, ParsingOutcome};
use crate::compiler::parsable::codex::Codex;
use crate::compiler::parsable::parsing_configuration::ParsingConfiguration;
use crate::compiler::parsable::{codex::parsing_rule::parsing_outcome::ParsingError, Parsable};


#[derive(Debug)]
pub struct Chapter {
    heading: Heading,
    paragraphs: Vec<Paragraph>

    // TODO: maybe in another version
    /* subchapters: Option<Vec<Arc<Chapter>>>,
    superchapter: Option<Arc<Chapter>> */
}

#[allow(dead_code)]
impl Chapter {

    pub fn new(heading: Heading, paragraphs: Vec<Paragraph>) -> Self {
        Self {
            heading,
            paragraphs
        }
    }

    pub fn heading(&self) -> &Heading {
        &self.heading
    }

    pub fn set_heading(&mut self, heading: &Heading) -> () {
        self.heading = heading.clone()
    }

    pub fn paragraphs(&self) -> &Vec<Paragraph> {
        &self.paragraphs
    }

    pub fn set_paragraphs(&mut self, paragraphs: Vec<Paragraph>) {
        self.paragraphs = paragraphs
    }
}

impl Clone for Chapter {
    fn clone(&self) -> Self {
        Self { heading: self.heading.clone(), paragraphs: self.paragraphs.clone() }
    }
}


impl Parsable for Chapter {
    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError> {

        // TODO

        let parsing_outcome = ParsingOutcome::new(codex.parse_text(&self.heading, Arc::clone(&parsing_configuration))?.parsed_content());


        log::debug!("parsing chapter:\n{:#?}", self);

        if parsing_configuration.parallelization() {

            let maybe_failed = self.paragraphs.par_iter_mut()
                .map(|paragraph| {
                    let result = paragraph.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration));

                    if let Ok(result) = result {
                        parsing_outcome.append_parsed_content(&result.parsed_content())
                    }

                    result.map(|r| ())
                })
                .find_any(|result| result.is_err());
    
            if let Some(result) = maybe_failed {
                return Err(result.err().unwrap());
            }

        } else {
            
            let maybe_failed = self.paragraphs.iter_mut()
                .map(|paragraph| {
                    let result = paragraph.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration));

                    if let Ok(result) = result {
                        parsing_outcome.append_parsed_content(&result.parsed_content())
                    }

                    result.map(|r| ())
                })
                .find(|result| result.is_err());
    
            if let Some(result) = maybe_failed {
                return Err(result.err().unwrap());
            }
        }

        Ok(parsing_outcome)
    }
}


impl Display for Chapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let mut s: String = String::from(&self.heading);

        for paragraph in &self.paragraphs {
            s.push_str(paragraph.to_string().as_str());
        }

        write!(f, "{}", s)
    }
}

/* impl ToString for Chapter {
    fn to_string(&self) -> String {
        format!("{}", self)
    }
} */