pub mod document;
pub mod dossier_configuration;

use std::{path::PathBuf, sync::{Arc, RwLock}, time::Instant};

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
    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>, parsing_metadata: Arc<ParsingMetadata>) -> Result<(), ParsingError> {

        log::info!("parse dossier {} with {} document(s) (parallelization: {})", self.name(), self.documents().len(), parsing_configuration.parallelization());

        let mut parsing_metadata = parsing_metadata.as_ref().clone();

        parsing_metadata.set_dossier_name(Some(self.name().clone()));

        let parsing_metadata = Arc::new(parsing_metadata);

        if parsing_configuration.parallelization() {

            let maybe_fails = self.documents.par_iter_mut()
                .map(|document| {

                    let parse_time = Instant::now();

                    let res = document.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration), Arc::clone(&parsing_metadata));

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

                    let res = document.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration), Arc::clone(&parsing_metadata));

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
