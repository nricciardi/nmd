use std::{collections::HashMap, fmt::Debug, sync::{Arc, RwLock}};

use regex::{Captures, Regex};

use crate::compiler::{codex::{modifier::standard_text_modifier::StandardTextModifier, Codex}, parsing::{parsing_configuration::ParsingConfiguration, parsing_error::ParsingError, parsing_outcome::ParsingOutcome}};

use super::ParsingRule;


pub struct ReferenceRule {
    searching_pattern: String
}

impl ReferenceRule {
    pub fn new() -> Self {
        Self {
            searching_pattern: StandardTextModifier::Reference.modifier_pattern()
        }
    }
}

impl Debug for ReferenceRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReferenceRule").field("searching_pattern", &self.searching_pattern).finish()
    }
}

impl ParsingRule for ReferenceRule {
    fn searching_pattern(&self) -> &String {
        &self.searching_pattern
    }

    fn standard_parse(&self, content: &str, codex: &Codex, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<ParsingOutcome, ParsingError> {
        
        let regex = Regex::new(&self.searching_pattern).unwrap();

        let parsed_content = regex.replace_all(content, |capture: &Captures| {

            let reference_key = capture.get(1).unwrap().as_str();

            if let Some(reference) = parsing_configuration.read().unwrap().references().get(reference_key) {
                return String::from(reference)
            } else {
                log::error!("reference '{}' ('{}') not found: no replacement will be applied", reference_key, capture.get(0).unwrap().as_str());
                return String::from(content);
            }
        });

        Ok(ParsingOutcome::new(parsed_content.to_string()))
    }
}
