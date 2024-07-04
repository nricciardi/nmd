use std::{collections::HashMap, fmt::Debug, sync::{Arc, RwLock}};

use regex::{Captures, Regex};

use crate::compiler::{codex::{modifier::standard_text_modifier::StandardTextModifier, Codex}, parsing::{parsing_configuration::ParsingConfiguration, parsing_error::ParsingError, parsing_outcome::ParsingOutcome}};

use super::ParsingRule;


pub struct HtmlCiteRule {
    search_pattern: String,
    search_pattern_regex: Regex,
}

impl HtmlCiteRule {
    pub fn new() -> Self {
        Self {
            search_pattern: StandardTextModifier::Cite.modifier_pattern(),
            search_pattern_regex: Regex::new(&StandardTextModifier::Cite.modifier_pattern()).unwrap(),
        }
    }
}

impl Debug for HtmlCiteRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CiteRule").field("searching_pattern", &self.search_pattern).finish()
    }
}

impl ParsingRule for HtmlCiteRule {

    fn search_pattern(&self) -> &String {
        &self.search_pattern
    }

    fn standard_parse(&self, content: &str, codex: &Codex, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<ParsingOutcome, ParsingError> {
        
        let parsed_content = self.search_pattern_regex.replace_all(content, |capture: &Captures| {

            let bib_key = capture.get(1).unwrap().as_str();

            if let Some(bibliography) = parsing_configuration.read().unwrap().bibliography() {
                
                if let Some(n) = bibliography.get_n_from_key(bib_key) {
                    if let Some(reference) = bibliography.get_reference_from_key(bib_key) {
                        if let Ok(reference) = reference {
                            return format!(r#"<a class="cite" href="{}">{}</a>"#, reference.build(), n);
                        }
                    }
                }

                log::error!("bibliography record with key: '{}' ('{}') not found: no replacement will be applied", bib_key, capture.get(0).unwrap().as_str());
                
            } else {

                log::error!("bibliography '{}' ('{}') not found: no replacement will be applied", bib_key, capture.get(0).unwrap().as_str());
            }

            return String::from(content);
        });

        Ok(ParsingOutcome::new_fixed(parsed_content.to_string()))
    }
    
    fn search_pattern_regex(&self) -> &Regex {
        &self.search_pattern_regex
    }
}
