use super::{modifiers_bucket::ModifiersBucket, Mod, ModifierIdentifier};



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

impl Mod for BaseModifier {
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