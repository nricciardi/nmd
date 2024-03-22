use super::modifier::{paragraph_modifier::ParagraphModifier, text_modifier::TextModifier, Mod};

#[derive(Debug)]
pub struct CodexConfiguration {
    ordered_text_modifiers: Vec<Box<dyn Mod>>,
    ordered_paragraph_modifiers: Vec<Box<dyn Mod>>,
}

impl CodexConfiguration {

    pub fn new(ordered_text_modifiers: Vec<Box<dyn Mod>>, ordered_paragraph_modifiers: Vec<Box<dyn Mod>>) -> Self {
        Self {
            ordered_text_modifiers,
            ordered_paragraph_modifiers
        }
    }

    pub fn ordered_text_modifiers(&self) -> &Vec<Box<dyn Mod>> {
        &self.ordered_text_modifiers
    }

    pub fn ordered_paragraph_modifiers(&self) -> &Vec<Box<dyn Mod>> {
        &self.ordered_paragraph_modifiers
    }
}

impl Default for CodexConfiguration {
    fn default() -> Self {
        Self {
            ordered_text_modifiers: Vec::from_iter(TextModifier::ordered().into_iter().map(|m| Box::new(m) as Box<dyn Mod>)),
            ordered_paragraph_modifiers: Vec::from_iter(ParagraphModifier::ordered().into_iter().map(|m| Box::new(m) as Box<dyn Mod>)),
        }
    }
}