use std::sync::Arc;

use thiserror::Error;

use super::{heading::Heading, paragraph::ParagraphError, Chapter, Paragraph};

#[derive(Error, Debug)]
pub enum ChapterBuilderError {
    #[error("impossible to build")]
    ImpossibleToBuild,

    #[error(transparent)]
    ParagraphError(#[from] ParagraphError)
}


pub struct ChapterBuilder {
    heading: Option<Heading>,
    paragraphs: Vec<Paragraph>,
}


#[allow(dead_code)]
impl ChapterBuilder {

    pub fn new() -> Self {
        Self {
            heading: Option::None,
            paragraphs: Vec::new()
        }
    }

    pub fn set_heading(&mut self, heading: Heading) -> () {
        self.heading = Option::Some(heading)
    }

    pub fn build(self) -> Result<Chapter, ChapterBuilderError> {

        if self.heading.is_none() {
            return Err(ChapterBuilderError::ImpossibleToBuild);
        }

        todo!()

        // Ok(Chapter {
        //     heading: self.heading.unwrap(),
        //     options: self.options,
        //     paragraphs: self.paragraphs
        // })
    }
}