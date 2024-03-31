use std::sync::Arc;

use thiserror::Error;

use crate::compiler::parsable::codex::Codex;

use super::{Chapter, Paragraph, paragraph::ParagraphError};

#[derive(Error, Debug)]
pub enum ChapterBuilderError {
    #[error("impossible to build")]
    ImpossibleToBuild,

    #[error(transparent)]
    ParagraphError(#[from] ParagraphError)
}


pub struct ChapterBuilder {
    codex: Arc<Codex>,
    heading: Option<String>,
    content: Option<String>,
}


#[allow(dead_code)]
impl ChapterBuilder {

    pub fn new(codex: Arc<Codex>) -> Self {
        Self {
            codex,
            heading: Option::None,
            content: Option::None
        }
    }

    pub fn new_with_heading(codex: Arc<Codex>, heading: String) -> Self {
        Self {
            codex,
            heading: Option::Some(heading),
            content: Option::None
        }
    }

    pub fn set_heading(&mut self, heading: String) -> () {
        self.heading = Option::Some(heading)
    }

    pub fn append_content(&mut self, new_content: String) -> () {
        if let Some(ref mut content) = self.content {
            content.push_str(&new_content);
        } else {
            self.content = Option::Some(new_content);
        }
    }

    pub fn build(self) -> Result<Chapter, ChapterBuilderError> {

        if self.heading.is_none() {
            return Err(ChapterBuilderError::ImpossibleToBuild);
        }

        let mut paragraphs: Vec<Paragraph> = Vec::new();

        if let Some(content) = &self.content {

            paragraphs = self.codex.str_to_paragraphs(content)?;
        }

        Ok(Chapter {
            heading: self.heading.unwrap(),
            paragraphs
        })
    }
}