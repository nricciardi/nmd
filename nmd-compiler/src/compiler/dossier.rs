mod document;
pub mod dossier_configuration;

use std::sync::Arc;

pub use document::{Document, DocumentError};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use thiserror::Error;

use crate::compiler::{parsable::Parsable, compilable::Compilable};


use self::dossier_configuration::DossierConfiguration;

use super::{loadable::{Loadable, LoadError}, parsable::{ParsingConfiguration, ParsingError, codex::Codex}, resource::{Resource, ResourceError}};

#[derive(Error, Debug)]
pub enum DossierError {
    #[error("dossier loading failed: '{0}'")]
    Load(&'static str)
}

pub struct Dossier {
    configuration: DossierConfiguration,
    documents: Option<Vec<Document>>
}

impl Loadable for Dossier {

    type Type = DossierConfiguration;

    fn load(dossier_configuration: &Self::Type) -> Result<Box<Self>, LoadError> {

        if dossier_configuration.documents().is_empty() {
            return Err(LoadError::ResourceError(ResourceError::InvalidResourceVerbose("there are no documents".to_string())))
        }

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
            documents: Option::Some(documents)
        }))
    }

}

impl Parsable for Dossier {
    fn parse(&mut self,codex: Arc<Codex>, parsing_configuration: Arc<ParsingConfiguration>) -> Result<(), ParsingError> {
        if let Some(documents) = &mut self.documents {
            let maybe_fails = documents.par_iter_mut().map(|document| {
                document.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration))  
            }).find_any(|result| result.is_err());

            if let Some(Err(fail)) = maybe_fails {
                return Err(fail)
            }
        }

        Ok(())
    }
}

impl Dossier {

    pub fn name(&self) -> &String {
        self.configuration.name()
    }
}
