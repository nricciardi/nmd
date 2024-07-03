pub mod document;
pub mod dossier_configuration;

use std::{borrow::{Borrow, BorrowMut}, path::PathBuf, sync::{Arc, RwLock}, time::Instant};

use document::chapter::{self, heading::Heading};
pub use document::{Document, DocumentError};
use rayon::{iter::{IntoParallelRefMutIterator, ParallelIterator}, slice::IterMut};
use thiserror::Error;

use crate::{compiler::table_of_contents::content_tree::{ContentTree, ContentTreeNode}, resource::ResourceError};

use self::dossier_configuration::DossierConfiguration;

use super::{codex::Codex, output_format::OutputFormat, parsable::Parsable, parsing::{parsing_configuration::{parsing_configuration_overlay::ParsingConfigurationOverLay, ParsingConfiguration}, parsing_error::ParsingError, parsing_metadata::ParsingMetadata}, table_of_contents::TableOfContents};


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
pub struct Dossier {
    configuration: DossierConfiguration,
    table_of_contents: Option<TableOfContents>,
    documents: Vec<Document>
}

impl Dossier {

    pub fn new(configuration: DossierConfiguration, documents: Vec<Document>) -> Self {

        log::warn!("{:#?}", configuration);

        Self {
            configuration,
            table_of_contents: None,
            documents
        }
    }

    pub fn name(&self) -> &String {
        self.configuration.name()
    }

    pub fn documents(&self) -> &Vec<Document> {
        &self.documents
    }

    pub fn configuration(&self) -> &DossierConfiguration {
        &self.configuration
    }

    pub fn table_of_contents(&self) -> &Option<TableOfContents> {
        &self.table_of_contents
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

        Ok(())
    }
}
