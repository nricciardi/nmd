mod document;
mod dossier_configuration;

use std::sync::Arc;

pub use document::{Document, DocumentError};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use thiserror::Error;

use crate::compiler::{parsable::Parsable, compilable::Compilable};


use self::dossier_configuration::DossierConfiguration;

use super::{compilable::{compilation_configuration::CompilationConfiguration, CompilationError}, loadable::{Loadable, LoadError}, parsable::{ParsingConfiguration, ParsingError, codex::Codex}, resource::Resource, assemblable::Assemblable, dumpable::Dumpable};

#[derive(Error, Debug)]
pub enum DossierError {
    #[error("dossier loading failed: '{0}'")]
    Load(&'static str)
}

pub struct Dossier {
    name: String,
    documents: Option<Vec<Document>>
}

impl Loadable for Dossier {

    type Type = DossierConfiguration;

    fn load(resource: Self::Type) -> Result<Box<Self>, LoadError> {
        todo!()
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

impl Assemblable for Dossier {
    
}

impl Dumpable for Dossier {
    
}

impl Compilable for Dossier {
    fn compile(&self, compilation_configuration: Arc<CompilationConfiguration>) -> Result<(), CompilationError> {
        todo!()
    }
}

impl Dossier {

    pub fn new(name: String, documents: Option<Vec<Document>>) -> Self {
        Self {
            name,
            documents
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
