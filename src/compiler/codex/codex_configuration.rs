use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use super::modifier::{base_modifier::BaseModifier, standard_chapter_modifier::StandardChapterModifier, standard_paragraph_modifier::StandardParagraphModifier, standard_text_modifier::StandardTextModifier, Modifier, ModifierIdentifier};

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

    pub fn chapter_modifier(&self, identifier: &ModifierIdentifier) -> Option<&Box<dyn Modifier>> {
        self.ordered_chapter_modifier().par_iter()
            .find_any(|paragraph_modifier| identifier.eq(paragraph_modifier.identifier()))
    }
}

impl Default for CodexConfiguration {
    fn default() -> Self {
        Self {
            ordered_text_modifiers: Vec::from_iter(StandardTextModifier::ordered().into_iter().map(|m| Box::new(Into::<BaseModifier>::into(m)) as Box<dyn Modifier>)),
            ordered_paragraph_modifiers: Vec::from_iter(StandardParagraphModifier::ordered().into_iter().map(|m| Box::new(Into::<BaseModifier>::into(m)) as Box<dyn Modifier>)),
            ordered_chapter_modifier: Vec::from_iter(StandardChapterModifier::ordered().into_iter().map(|m| Box::new(Into::<BaseModifier>::into(m)) as Box<dyn Modifier>)),
        }
    }
}