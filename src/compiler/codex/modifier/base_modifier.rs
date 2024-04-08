use super::{modifiers_bucket::ModifiersBucket, Modifier, ModifierIdentifier};



pub struct BaseModifier {
    identifier: ModifierIdentifier,
    search_pattern: String,
    incompatible_modifiers: ModifiersBucket,
}

impl BaseModifier {
    pub fn new(identifier: ModifierIdentifier, search_pattern: String, incompatible_modifiers: ModifiersBucket) -> Self {
        Self {
            identifier,
            search_pattern,
            incompatible_modifiers
        }
    }
}

impl Modifier for BaseModifier {
    fn identifier(&self) -> &ModifierIdentifier {
        &self.identifier
    }

    fn search_pattern(&self) -> &String {
        &self.search_pattern
    }

    fn incompatible_modifiers(&self) -> &ModifiersBucket {
        &self.incompatible_modifiers
    }
}