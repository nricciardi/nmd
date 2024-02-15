use serde::{Deserialize, Serialize};

pub const CHECKBOX: &str = ":checkbox:";
pub const CHECKBOX_CHECKED: &str = ":checkbox-checked:";


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListBulletConfigurationRecord {
    pub from: String,
    pub to: String,
    pub indentation_level: usize,
    pub strict_indentation: bool
}

impl ListBulletConfigurationRecord {

    pub fn new(from: String, to: String, indentation_level: usize, strict_indentation: bool) -> Self {
        Self {
            from,
            to,
            indentation_level,
            strict_indentation
        }
    }

    pub fn from(&self) -> &String {
        &self.from
    }

    pub fn to(&self) -> &String {
        &self.to
    }

    pub fn indentation_level(&self) -> usize {
        self.indentation_level
    }

    pub fn strict_indentation(&self) -> bool {
        self.strict_indentation
    }
}