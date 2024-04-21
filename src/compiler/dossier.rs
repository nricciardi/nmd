pub mod document;
pub mod dossier_configuration;

use std::{borrow::{Borrow, BorrowMut}, path::PathBuf, sync::{Arc, RwLock}, time::Instant};

pub use document::{Document, DocumentError};
use rayon::{iter::{IntoParallelRefMutIterator, ParallelIterator}, slice::IterMut};
use thiserror::Error;

use crate::resource::ResourceError;

use self::dossier_configuration::DossierConfiguration;

use super::{codex::Codex, parsable::Parsable, parsing::{parsing_configuration::ParsingConfiguration, parsing_error::ParsingError, parsing_metadata::ParsingMetadata}};


pub const ASSETS_DIR: &str = "assets";
pub const IMAGES_DIR: &str = "images";
pub const DOCUMENTS_DIR: &str = "documents";
pub const STYLES_DIR: &str = "styles";

#[derive(Error, Debug)]
pub enum DossierError {
    #[error("dossier loading failed: '{0}'")]
    Load(#[from] ResourceError)
}

pub struct Dossier {
    configuration: DossierConfiguration,
    documents: Vec<Document>
}

impl Dossier {

    pub fn new(configuration: DossierConfiguration, documents: Vec<Document>) -> Self {
        Self {
            configuration,
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
}


impl Parsable for Dossier {
    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<(), ParsingError> {

        let parallelization = parsing_configuration.read().unwrap().parallelization();

        log::info!("parse dossier {} with {} document(s) (parallelization: {})", self.name(), self.documents().len(), parallelization);

        parsing_configuration.write().unwrap().metadata_mut().set_dossier_name(Some(self.name().clone()));

        if parallelization {

            let maybe_fails = self.documents.par_iter_mut()
                .map(|document| {

                    let parse_time = Instant::now();

                    let new_parsing_configuration: Arc<RwLock<ParsingConfiguration>> = Arc::new(RwLock::new(parsing_configuration.read().unwrap().clone()));

                    // Arc::new because parallelization on (may be override during multi-thread operations)
                    let res = document.parse(Arc::clone(&codex), new_parsing_configuration);

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

                    let res = document.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration));

                    log::info!("document '{}' parsed in {} ms", document.name(), parse_time.elapsed().as_millis());

                    res
                })
                .find(|result| result.is_err());

                if let Some(Err(fail)) = maybe_fails {
                    return Err(fail)
                }
        }

        Ok(())
    }
}
