pub mod paragraph;
pub mod heading;
pub mod chapter_builder;
pub mod chapter_tag;

use std::fmt::Display;
use std::sync::{Arc, RwLock};
use std::thread;

use chapter_tag::ChapterTag;
use getset::{Getters, Setters};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

use crate::compiler::codex::Codex;
use crate::compiler::output_format::OutputFormat;
use crate::compiler::parsable::Parsable;
use crate::compiler::parsing::parsing_configuration::parsing_configuration_overlay::ParsingConfigurationOverLay;
use crate::compiler::parsing::parsing_configuration::ParsingConfiguration;
use crate::compiler::parsing::parsing_error::ParsingError;
use crate::compiler::parsing::parsing_metadata::ParsingMetadata;

use self::heading::Heading;
pub use self::paragraph::Paragraph;


#[derive(Debug, Getters, Setters)]
pub struct Chapter {

    #[getset(get = "pub", set = "pub")]
    heading: Heading,

    #[getset(get = "pub", set = "pub")]
    tags: Vec<ChapterTag>,
    
    #[getset(get = "pub", set = "pub")]
    paragraphs: Vec<Paragraph>,
}


impl Chapter {

    pub fn new(heading: Heading, tags: Vec<ChapterTag>, paragraphs: Vec<Paragraph>) -> Self {
        Self {
            heading,
            tags,
            paragraphs
        }
    }
}


impl Parsable for Chapter {
    fn standard_parse(&mut self, format: &OutputFormat, codex: Arc<Codex>, parsing_configuration: Arc<RwLock<ParsingConfiguration>>, parsing_configuration_overlay: Arc<Option<ParsingConfigurationOverLay>>) -> Result<(), ParsingError> {

        self.heading.parse(format, Arc::clone(&codex), Arc::clone(&parsing_configuration), Arc::clone(&parsing_configuration_overlay))?;

        log::debug!("parsing chapter:\n{:#?}", self);

        if parsing_configuration.read().unwrap().parallelization() {

            let maybe_failed = self.paragraphs.par_iter_mut()
                .map(|paragraph| {
                    paragraph.parse(format, Arc::clone(&codex), Arc::clone(&parsing_configuration), Arc::clone(&parsing_configuration_overlay))
                })
                .find_any(|result| result.is_err());
    
            if let Some(result) = maybe_failed {
                return result
            }

        } else {
            
            let maybe_failed = self.paragraphs.iter_mut()
                .map(|paragraph| {
                    paragraph.parse(format, Arc::clone(&codex), Arc::clone(&parsing_configuration), Arc::clone(&parsing_configuration_overlay))
                })
                .find(|result| result.is_err());
    
            if let Some(result) = maybe_failed {
                return result
            }
        }

        Ok(())
    }
}

// impl Display for Chapter {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

//         let mut s: String = String::from(&self.heading);

//         for paragraph in &self.paragraphs {
//             s.push_str(paragraph.to_string().as_str());
//         }

//         write!(f, "{}", s)
//     }
// }

/* impl ToString for Chapter {
    fn to_string(&self) -> String {
        format!("{}", self)
    }
} */