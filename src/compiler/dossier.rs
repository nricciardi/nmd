pub mod document;
pub mod dossier_configuration;

use std::{path::PathBuf, sync::Arc, time::Instant};

pub use document::{Document, DocumentError};
use rayon::{iter::{IntoParallelRefMutIterator, ParallelIterator}, slice::IterMut};
use thiserror::Error;

use crate::resource::ResourceError;

use self::dossier_configuration::DossierConfiguration;

use super::{codex::Codex, parser::{parsable::Parsable, parsing_rule::{parsing_configuration::ParsingConfiguration, parsing_error::ParsingError}}};


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

// impl Loadable<PathBuf> for Dossier {

//     fn load(codex: Arc<Codex>, location: &PathBuf) -> Result<Box<Self>, LoadError> {

//         let dossier_configuration = DossierConfiguration::try_from(location)?;

//         Self::load(Arc::clone(&codex), &dossier_configuration)
//     }

// }

// impl Loadable<DossierConfiguration> for Dossier {
//     fn load(codex: Arc<Codex>, dossier_configuration: &DossierConfiguration) -> Result<Box<Self>, LoadError> {
//         // TODO: are really mandatory?
//         if dossier_configuration.raw_documents_paths().is_empty() {
//             return Err(LoadError::ResourceError(ResourceError::InvalidResourceVerbose("there are no documents".to_string())))
//         }

//         // TODO: is really mandatory?
//         if dossier_configuration.name().is_empty() {
//             return Err(LoadError::ResourceError(ResourceError::InvalidResourceVerbose("there is no name".to_string())))
//         }

//         let mut documents: Vec<Document> = Vec::new();

//         for document in dossier_configuration.raw_documents_paths() {

//             let document = Document::load(Arc::clone(&codex), document)?;

//             documents.push(*document)
//         }

//         Ok(Box::new(Self {
//             configuration: dossier_configuration.clone(),
//             documents: documents
//         }))
//     }
// }


impl Parsable for Dossier {
    fn parse(&mut self, codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError> {

        log::info!("parse dossier {} with {} document(s) (parallelization: {})", self.name(), self.documents().len(), parsing_configuration.parallelization());

        if parsing_configuration.parallelization() {

            let maybe_fails = self.documents.par_iter_mut()
                .map(|document| {
                    let parse_time = Instant::now();

                    let res = document.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration));

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
