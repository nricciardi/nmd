pub mod chapter;

pub use chapter::Chapter;

use crate::compiler::{location::{Locatable, Location}, parsable::Parsable, compilable::Compilable};

pub struct Document {
    location: Location,
    name: String,
    chapters: Option<Vec<Chapter> >
}

impl Locatable for Document {
    fn location(self: &Self) -> &Location {
        &self.location
    }
}

/* impl Parsable for Document {
    fn parse(&self) {
        todo!()
    }
}

impl Compilable for Document {
    fn compile(&self) -> anyhow::Result<()> {
        todo!()
    }
} */