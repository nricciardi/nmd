use super::{modifiers_bucket::ModifiersBucket, Modifier, ModifierIdentifier, ModifierPattern};



pub struct BaseModifier {
    identifier: ModifierIdentifier,
    modifier_pattern: ModifierPattern,
    incompatible_modifiers: ModifiersBucket,
}

impl BaseModifier {
    pub fn new(identifier: ModifierIdentifier, modifier_pattern: ModifierPattern, incompatible_modifiers: ModifiersBucket) -> Self {
        Self {
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
}