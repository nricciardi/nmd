pub mod chapter;

use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::time::Instant;

pub use chapter::Chapter;
use thiserror::Error;
use log;
use rayon::prelude::*;

use crate::compiler::codex::Codex;
use crate::compiler::output_format::OutputFormat;
use crate::compiler::parsable::Parsable;
use crate::compiler::parsing::parsing_configuration::parsing_configuration_overlay::ParsingConfigurationOverLay;
use crate::compiler::parsing::parsing_configuration::ParsingConfiguration;
use crate::compiler::parsing::parsing_error::ParsingError;
use crate::compiler::parsing::parsing_metadata::ParsingMetadata;
use crate::resource::disk_resource::DiskResource;
use crate::resource::{Resource, ResourceError};

use self::chapter::paragraph::ParagraphError;
pub use self::chapter::Paragraph;
use self::chapter::chapter_builder::{ChapterBuilder, ChapterBuilderError};


#[derive(Error, Debug)]
pub enum DocumentError {
    #[error(transparent)]
    Load(#[from] ResourceError),

    #[error(transparent)]
    Parsing(#[from] ParsingError),

    #[error(transparent)]
    ChapterBuilderError(#[from] ChapterBuilderError),

    #[error(transparent)]
    ParagraphError(#[from] ParagraphError),
}

pub struct Document {
    name: String,
    preamble: Vec<Paragraph>,
    chapters: Vec<Chapter>
}


#[allow(dead_code)]
impl Document {

    pub fn new(name: String, preamble: Vec<Paragraph>, chapters: Vec<Chapter>) -> Self {
        
        Self {
            name,
            preamble,
            chapters
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    } 

    pub fn chapters(&self) -> &Vec<Chapter> {
        &self.chapters
    }

    pub fn preamble(&self) -> &Vec<Paragraph> {
        &self.preamble
    }
}

impl Parsable for Document {

    fn standard_parse(&mut self, format: &OutputFormat, codex: Arc<Codex>, parsing_configuration: Arc<RwLock<ParsingConfiguration>>, parsing_configuration_overlay: Arc<Option<ParsingConfigurationOverLay>>) -> Result<(), ParsingError> {

        let parallelization = parsing_configuration.read().unwrap().parallelization();

        log::info!("parsing {} chapters of document: '{}'", self.chapters().len(), self.name);

        parsing_configuration.write().unwrap().metadata_mut().set_document_name(Some(self.name().clone()));

        if parallelization {

            let maybe_one_failed: Option<Result<(), ParsingError>> = self.preamble.par_iter_mut()
                .map(|paragraph| {

                    paragraph.parse(format, Arc::clone(&codex), Arc::clone(&parsing_configuration), Arc::clone(&parsing_configuration_overlay))
                
                }).find_any(|result| result.is_err());

            if let Some(result) = maybe_one_failed {
                return result;
            }

            let maybe_one_failed: Option<Result<(), ParsingError>> = self.chapters.par_iter_mut()
                .map(|chapter| {

                    chapter.parse(format, Arc::clone(&codex), Arc::clone(&parsing_configuration), Arc::clone(&parsing_configuration_overlay))
                
                }).find_any(|result| result.is_err());

            if let Some(result) = maybe_one_failed {
                return result;
            }
        
        } else {

            let maybe_one_failed: Option<Result<(), ParsingError>> = self.preamble.iter_mut()
                .map(|paragraph| {

                    paragraph.parse(format, Arc::clone(&codex), Arc::clone(&parsing_configuration), Arc::clone(&parsing_configuration_overlay))
                
                }).find(|result| result.is_err());

            if let Some(result) = maybe_one_failed {
                return result;
            }
            
            let maybe_one_failed: Option<Result<(), ParsingError>> = self.chapters.iter_mut()
                .map(|chapter| {

                    chapter.parse(format, Arc::clone(&codex), Arc::clone(&parsing_configuration), Arc::clone(&parsing_configuration_overlay))
                
                }).find(|result| result.is_err());

            if let Some(result) = maybe_one_failed {
                return result;
            }
        }

        Ok(())

    }
}

