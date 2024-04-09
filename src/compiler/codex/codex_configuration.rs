use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::modifier::{base_modifier::BaseModifier, chapter_modifier::ChapterModifier, paragraph_modifier::ParagraphModifier, text_modifier::TextModifier, Modifier, ModifierIdentifier};

#[derive(Debug)]
pub struct CodexConfiguration {
    ordered_text_modifiers: Vec<Box<dyn Modifier>>,
    ordered_paragraph_modifiers: Vec<Box<dyn Modifier>>,
    ordered_chapter_modifier: Vec<Box<dyn Modifier>>,
}

impl CodexConfiguration {

    pub fn new(ordered_text_modifiers: Vec<Box<dyn Modifier>>, ordered_paragraph_modifiers: Vec<Box<dyn Modifier>>, ordered_chapter_modifier: Vec<Box<dyn Modifier>>) -> Self {
        Self {
            ordered_text_modifiers,
            ordered_paragraph_modifiers,
            ordered_chapter_modifier
        }
    }


    pub fn ordered_text_modifiers(&self) -> &Vec<Box<dyn Modifier>> {
        &self.ordered_text_modifiers
    }

    pub fn text_modifier(&self, identifier: &ModifierIdentifier) -> Option<&Box<dyn Modifier>> {
        self.ordered_text_modifiers().par_iter()
            .find_any(|paragraph_modifier| identifier.eq(paragraph_modifier.identifier()))
    }

    pub fn ordered_paragraph_modifiers(&self) -> &Vec<Box<dyn Modifier>> {
        &self.ordered_paragraph_modifiers
    }

    pub fn paragraph_modifier(&self, identifier: &ModifierIdentifier) -> Option<&Box<dyn Modifier>> {
        self.ordered_paragraph_modifiers().par_iter()
            .find_any(|paragraph_modifier| identifier.eq(paragraph_modifier.identifier()))
    }

    pub fn ordered_chapter_modifier(&self) -> &Vec<Box<dyn Modifier>> {
        &self.ordered_chapter_modifier
    }
}

impl Default for CodexConfiguration {
    fn default() -> Self {
        Self {
            ordered_text_modifiers: Vec::from_iter(TextModifier::ordered().into_iter().map(|m| Box::new(Into::<BaseModifier>::into(m)) as Box<dyn Modifier>)),
            ordered_paragraph_modifiers: Vec::from_iter(ParagraphModifier::ordered().into_iter().map(|m| Box::new(Into::<BaseModifier>::into(m)) as Box<dyn Modifier>)),
            ordered_chapter_modifier: Vec::from_iter(ChapterModifier::ordered().into_iter().map(|m| Box::new(Into::<BaseModifier>::into(m)) as Box<dyn Modifier>)),
        }
    }
}