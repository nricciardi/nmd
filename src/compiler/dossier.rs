pub mod document;
pub mod dossier_configuration;

use std::{sync::{Arc, RwLock}, time::Instant};

use document::chapter::heading::Heading;
pub use document::{Document, DocumentError};
use getset::{Getters, Setters};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use thiserror::Error;

use crate::resource::ResourceError;

use self::dossier_configuration::DossierConfiguration;

use super::{bibliography::Bibliography, codex::Codex, output_format::OutputFormat, parsable::Parsable, parsing::{parsing_configuration::{parsing_configuration_overlay::ParsingConfigurationOverLay, ParsingConfiguration}, parsing_error::ParsingError}, table_of_contents::TableOfContents};


pub const ASSETS_DIR: &str = "assets";
pub const IMAGES_DIR: &str = "images";
pub const DOCUMENTS_DIR: &str = "documents";
pub const STYLES_DIR: &str = "styles";


#[derive(Error, Debug)]
pub enum DossierError {
    #[error("dossier loading failed: '{0}'")]
    Load(#[from] ResourceError)
}


/// NMD Dossier struct. It has own documents list
#[derive(Debug, Getters, Setters)]
pub struct Dossier {

    #[getset(get = "pub", set = "pub")]
    configuration: DossierConfiguration,

    #[getset(get = "pub", set = "pub")]
    table_of_contents: Option<TableOfContents>,

    #[getset(get = "pub", set = "pub")]
    documents: Vec<Document>,

    #[getset(get = "pub", set = "pub")]
    bibliography: Option<Bibliography>,
}

impl Dossier {

    pub fn new(configuration: DossierConfiguration, documents: Vec<Document>) -> Self {

        Self {
            configuration,
            table_of_contents: None,
            documents,
            bibliography: None,
        }
    }

    pub fn name(&self) -> &String {
        self.configuration.name()
    }
}


impl Parsable for Dossier {

    fn standard_parse(&mut self, format: &OutputFormat, codex: Arc<Codex>, parsing_configuration: Arc<RwLock<ParsingConfiguration>>, parsing_configuration_overlay: Arc<Option<ParsingConfigurationOverLay>>) -> Result<(), ParsingError> {

        let parallelization = parsing_configuration.read().unwrap().parallelization();

        log::info!("parse dossier {} with {} document(s) (parallelization: {})", self.name(), self.documents().len(), parallelization);

        parsing_configuration.write().unwrap().metadata_mut().set_dossier_name(Some(self.name().clone()));

        if parallelization {

            let maybe_fails = self.documents.par_iter_mut()
                .map(|document| {

                    let parse_time = Instant::now();

                    let new_parsing_configuration: Arc<RwLock<ParsingConfiguration>> = Arc::new(RwLock::new(parsing_configuration.read().unwrap().clone()));

                    // Arc::new because parallelization on (may be override during multi-thread operations)
                    let res = document.parse(format, Arc::clone(&codex), new_parsing_configuration, Arc::clone(&parsing_configuration_overlay));

                    log::info!("document '{}' parsed in {} ms", document.name(), parse_time.elapsed().as_millis());

                    res
                })
                .find_any(|result| result.is_err());

                if let Some(Err(fail)) = maybe_fails {
                    return Err(fail)
                }
            
        } else {
            let maybe_fails = self.documents.iter_mut()
                .map(|document| {
                    let parse_time = Instant::now();

                    let res = document.parse(format, Arc::clone(&codex), Arc::clone(&parsing_configuration), Arc::clone(&parsing_configuration_overlay));

                    log::info!("document '{}' parsed in {} ms", document.name(), parse_time.elapsed().as_millis());

                    res
                })
                .find(|result| result.is_err());

                if let Some(Err(fail)) = maybe_fails {
                    return Err(fail)
                }
        }

        if self.configuration.table_of_contents_configuration().include_in_output() {

            log::info!("dossier table of contents will be included in output");

            let mut headings: Vec<Heading> = Vec::new();

            for document in self.documents() {
                for chapter in document.chapters() {
                    headings.push(chapter.heading().clone());
                }
            }

            let mut table_of_contents = TableOfContents::new(
                self.configuration.table_of_contents_configuration().title().clone(),
                self.configuration.table_of_contents_configuration().page_numbers(),
                self.configuration.table_of_contents_configuration().plain(),
                self.configuration.table_of_contents_configuration().maximum_heading_level(),
                headings
            );

            table_of_contents.parse(format, Arc::clone(&codex), Arc::clone(&parsing_configuration), Arc::clone(&parsing_configuration_overlay))?;
        
            self.table_of_contents = Some(table_of_contents);
        }

        if self.configuration.bibliography().include_in_output() {
            let mut bibliography = Bibliography::new(
                self.configuration.bibliography().title().clone(),
                self.configuration.bibliography().records().clone()
            );

            bibliography.parse(format, Arc::clone(&codex), Arc::clone(&parsing_configuration), Arc::clone(&parsing_configuration_overlay))?;
        
            self.bibliography = Some(bibliography);
        }

        Ok(())
    }
}
