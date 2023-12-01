mod document;

pub use document::Document;

use crate::compiler::{location::{Location, Locatable}, parsable::Parsable, compilable::Compilable};

pub struct Dossier {
    location: Location,
    name: String,
    documents: Option<Vec<Document>>
}

impl Locatable for Dossier {
    fn location(self: &Self) -> &Location {
        &self.location
    }
}

impl Parsable for Dossier {
    fn parse(&self, parsing_configuration: &document::chapter::ParsingConfiguration) -> document::chapter::ParsingResult {
        todo!()
    }
}

impl Compilable for Dossier {
    fn compile(&self, compilation_configuration: crate::compiler::compilable::CompilationConfiguration) -> anyhow::Result<()> {
        todo!()
    }
}

impl Dossier {
    fn new(location: Location) -> Self {
        todo!()
        // Dossier { location: location, name: (), documents: () }
    }
}
