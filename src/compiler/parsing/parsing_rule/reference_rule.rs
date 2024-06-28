use std::{collections::HashMap, fmt::Debug, sync::{Arc, RwLock}};

use regex::{Captures, Regex};

use crate::compiler::{codex::{modifier::standard_text_modifier::StandardTextModifier, Codex}, parsing::{parsing_configuration::ParsingConfiguration, parsing_error::ParsingError, parsing_outcome::ParsingOutcome}};

use super::ParsingRule;


pub struct ReferenceRule {
    search_pattern: String,
    search_pattern_regex: Regex,
}

impl ReferenceRule {
    pub fn new() -> Self {
        Self {
            search_pattern: StandardTextModifier::Reference.modifier_pattern(),
            search_pattern_regex: Regex::new(&StandardTextModifier::Reference.modifier_pattern()).unwrap(),
        }
    }
}

impl Debug for ReferenceRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReferenceRule").field("searching_pattern", &self.search_pattern).finish()
    }
}

impl ParsingRule for ReferenceRule {

    fn search_pattern(&self) -> &String {
        &self.search_pattern
    }

    fn standard_parse(&self, content: &str, codex: &Codex, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<ParsingOutcome, ParsingError> {
        
        let parsed_content = self.search_pattern_regex.replace_all(content, |capture: &Captures| {

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
    
    fn search_pattern_regex(&self) -> &Regex {
        &self.search_pattern_regex
    }
}
