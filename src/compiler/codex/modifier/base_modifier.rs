use regex::Regex;

use super::{modifiers_bucket::ModifiersBucket, Modifier, ModifierIdentifier, ModifierPattern};



pub struct BaseModifier {
    identifier: ModifierIdentifier,
    modifier_pattern: ModifierPattern,
    incompatible_modifiers: ModifiersBucket,
    modifier_pattern_regex: Regex,
}

impl BaseModifier {
    pub fn new(identifier: ModifierIdentifier, modifier_pattern: ModifierPattern, incompatible_modifiers: ModifiersBucket) -> Self {
        Self {
            modifier_pattern_regex: Regex::new(&modifier_pattern).unwrap(),
            identifier,
            modifier_pattern,
            incompatible_modifiers
        }
    }
}

impl Modifier for BaseModifier {

    fn identifier(&self) -> &ModifierIdentifier {
        &self.identifier
    }

    fn modifier_pattern(&self) -> &ModifierPattern {
        &self.modifier_pattern
    }

    fn incompatible_modifiers(&self) -> &ModifiersBucket {
        &self.incompatible_modifiers
    }
    
    fn modifier_pattern_regex(&self) -> &Regex {
        &self.modifier_pattern_regex
    }
}