use std::{error, str::FromStr};

use thiserror::Error;

use super::{Chapter, Paragraph, paragraph};

#[derive(Error, Debug)]
pub enum ChapterBuilderError {
    #[error("impossible to build")]
    ImpossibleToBuild
}


pub struct ChapterBuilder {
    heading: Option<String>,
    content: Option<String>
}

impl ChapterBuilder {
    pub fn new() -> Self {
        Self {
            heading: Option::None,
            content: Option::None
        }
    }

    pub fn new_with_heading(heading: String) -> Self {
        Self {
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

        let mut paragraphs: Option<Vec<Paragraph>> = Option::None;

        if let Some(content) = &self.content {
            paragraphs = Paragraph::from_str(content)
        }

        Ok(Chapter {
            heading: self.heading.unwrap(),
            paragraphs
        })
    }
}