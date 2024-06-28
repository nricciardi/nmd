pub mod standard_paragraph_modifier;
pub mod modifiers_bucket;
pub mod standard_text_modifier;
pub mod standard_chapter_modifier;
pub mod base_modifier;
pub mod constants;

use std::fmt;

use regex::Regex;

use self::{base_modifier::BaseModifier, modifiers_bucket::ModifiersBucket};



pub type ModifierIdentifier = String;
pub type ModifierPattern = String;


/// `Modifier` is the component to identify a NMD modifier, which will be replaced using particular rule indicated by `Codex` 
pub trait Modifier: Sync + Send {

    fn identifier(&self) -> &ModifierIdentifier {
        &self.modifier_pattern()
    }

    fn modifier_pattern(&self) -> &ModifierPattern;
    
    fn modifier_pattern_regex(&self) -> &Regex; 

    fn incompatible_modifiers(&self) -> &ModifiersBucket {
        &ModifiersBucket::None
    }
}

impl fmt::Debug for dyn Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.identifier(), self.modifier_pattern())
    }
}

impl PartialEq for dyn Modifier {
    fn eq(&self, other: &Self) -> bool {
        self.modifier_pattern().eq(other.modifier_pattern())
    }
}

impl Clone for Box<dyn Modifier> {
    fn clone(&self) -> Self {
        Box::new(BaseModifier::new(self.identifier().clone(), self.modifier_pattern().clone(), self.incompatible_modifiers().clone()))
    }
}
