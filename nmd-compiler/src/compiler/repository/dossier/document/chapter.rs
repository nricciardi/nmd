pub mod paragraph;

use crate::compiler::parsable::Parsable;

pub use self::paragraph::Paragraph;

pub struct Chapter {
    title: String,
    paragraphs: Option<Vec<Paragraph>>,
    subchapters: Option<Vec<Box<Chapter>>>
}

impl Parsable for Chapter {
    fn parse(&self) {
        todo!()
    }
}