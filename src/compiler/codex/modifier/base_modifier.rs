use super::{modifiers_bucket::ModifiersBucket, Modifier, ModifierIdentifier};



pub struct BaseModifier {
    identifier: ModifierIdentifier,
    searching_pattern: String,
    incompatible_modifiers: ModifiersBucket,
}

impl BaseModifier {
    pub fn new(identifier: ModifierIdentifier, searching_pattern: String, incompatible_modifiers: ModifiersBucket) -> Self {
        Self {
            identifier,
            searching_pattern,
            incompatible_modifiers
        }
    }
}

impl Modifier for BaseModifier {
    fn identifier(&self) -> &ModifierIdentifier {
        &self.identifier
    }

    fn searching_pattern(&self) -> &String {
        &self.searching_pattern
    }

    fn incompatible_modifiers(&self) -> &ModifiersBucket {
        &self.incompatible_modifiers
    }
}