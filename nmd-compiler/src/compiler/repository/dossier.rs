mod document;

pub use document::Document;

use crate::compiler::{location::{Location, Locatable}, codex::parsable::Parsable, compilable::Compilable};

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

/* impl Parsable for Dossier {
    fn parse(&self) {
        todo!()
    }
}

impl Compilable for Dossier {
    fn compile(&self) -> anyhow::Result<()> {
        todo!()
    }
} */
