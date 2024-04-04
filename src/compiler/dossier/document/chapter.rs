pub mod paragraph;
pub mod heading;
pub mod chapter_builder;
pub mod chapter_options;

use std::fmt::Display;
use std::sync::Arc;

use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::compiler::codex::Codex;
use crate::compiler::parser::parsable::Parsable;
use crate::compiler::parser::parsing_rule::parsing_configuration::ParsingConfiguration;
use crate::compiler::parser::parsing_rule::parsing_error::ParsingError;

use self::chapter_options::ChapterOptions;
use self::heading::Heading;
pub use self::paragraph::Paragraph;


#[derive(Debug)]
pub struct Chapter {
    // heading: Heading,
    // options: ChapterOptions,
    heading: String,
    paragraphs: Vec<Paragraph>,


    // TODO: maybe in another version
    /* subchapters: Option<Vec<Arc<Chapter>>>,
    superchapter: Option<Arc<Chapter>> */
}

#[allow(dead_code)]
impl Chapter {

    pub fn new(heading: String /*: Heading, options: ChapterOptions*/, paragraphs: Vec<Paragraph>) -> Self {
        Self {
            heading,
            // options,
            paragraphs
        }
    }

    pub fn heading(&self) -> &String /*&Heading*/ {
        &self.heading
    }

    pub fn set_heading(&mut self, heading: String /*Heading*/) -> () {
        self.heading = heading
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
        Self {
            heading: self.heading.clone(),
            // options: self.options.clone(),
            paragraphs: self.paragraphs.clone()
        }
    }
}


impl Parsable for Chapter {
    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError> {
        self.heading = String::from(codex.parse_content(&self.heading, Arc::clone(&parsing_configuration))?.parsed_content());

        log::debug!("parsing chapter:\n{:#?}", self);

        if parsing_configuration.parallelization() {

            let maybe_failed = self.paragraphs.par_iter_mut()
                .map(|paragraph| {
                    paragraph.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration))
                })
                .find_any(|result| result.is_err());
    
            if let Some(result) = maybe_failed {
                return result
            }

        } else {
            
            let maybe_failed = self.paragraphs.iter_mut()
                .map(|paragraph| {
                    paragraph.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration))
                })
                .find(|result| result.is_err());
    
            if let Some(result) = maybe_failed {
                return result
            }
        }

        Ok(())
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