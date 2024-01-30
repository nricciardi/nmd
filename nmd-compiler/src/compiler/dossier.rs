pub mod document;
pub mod dossier_configuration;

use std::{sync::Arc, path::PathBuf, io, str::FromStr};

pub use document::{Document, DocumentError};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use thiserror::Error;

use crate::compiler::parsable::Parsable;
use self::dossier_configuration::DossierConfiguration;

use super::{loadable::{Loadable, LoadError}, parsable::{ParsingConfiguration, ParsingError, codex::Codex}, resource::{Resource, ResourceError}, utility::file_utility};
use crate::compiler::utility::file_utility::read_file_content;

#[derive(Error, Debug)]
pub enum DossierError {
    #[error("dossier loading failed: '{0}'")]
    Load(&'static str)
}

pub struct Dossier {
    configuration: DossierConfiguration,
    documents: Vec<Document>
}

impl Loadable<PathBuf> for Dossier {

    fn load(location: &PathBuf) -> Result<Box<Self>, LoadError> {

        let dossier_configuration = match DossierConfiguration::try_from(location) {
            Ok(dc) => dc,
            Err(e) => return Err(LoadError::ResourceError(ResourceError::InvalidResourceVerbose(String::from(format!("invalid dossier configuration: {}", e.to_string())))))
        };

        Self::load(&dossier_configuration)
    }

}

impl Loadable<DossierConfiguration> for Dossier {
    fn load(dossier_configuration: &DossierConfiguration) -> Result<Box<Self>, LoadError> {
        // TODO: are really mandatory?
        if dossier_configuration.documents().is_empty() {
            return Err(LoadError::ResourceError(ResourceError::InvalidResourceVerbose("there are no documents".to_string())))
        }

        // TODO: is really mandatory?
        if dossier_configuration.name().is_empty() {
            return Err(LoadError::ResourceError(ResourceError::InvalidResourceVerbose("there is no name".to_string())))
        }

        let mut documents: Vec<Document> = Vec::new();

        for document in dossier_configuration.documents() {

            let document = Document::load(document)?;

            documents.push(*document)
        }

        Ok(Box::new(Self {
            configuration: dossier_configuration.clone(),
            documents: documents
        }))
    }
}


impl Parsable for Dossier {
    fn parse(&mut self,codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError> {
        let maybe_fails = self.documents.par_iter_mut()
        .map(|document| {
            document.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration))  
        })
        .find_any(|result| result.is_err());

        if let Some(Err(fail)) = maybe_fails {
            return Err(fail)
        }

        Ok(())
    }
}

impl Dossier {

    pub fn name(&self) -> &String {
        self.configuration.name()
    }

    pub fn documents(&self) -> &Vec<Document> {
        &self.documents
    }
}
