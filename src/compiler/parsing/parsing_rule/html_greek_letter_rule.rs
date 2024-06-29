use std::{collections::HashMap, fmt::Debug, sync::{Arc, RwLock}};

use regex::{Captures, Regex};

use crate::compiler::{codex::{modifier::standard_text_modifier::StandardTextModifier, Codex}, parsing::{parsing_configuration::ParsingConfiguration, parsing_error::ParsingError, parsing_outcome::{ParsingOutcome, ParsingOutcomePart}}};

use super::ParsingRule;


pub struct HtmlGreekLettersRule {
    search_pattern: String,
    search_pattern_regex: Regex,
    greek_letters_map: HashMap<&'static str, &'static str>,
}

impl HtmlGreekLettersRule {
    pub fn new() -> Self {
        Self {
            search_pattern: StandardTextModifier::GreekLetter.modifier_pattern(),
            search_pattern_regex: Regex::new(&StandardTextModifier::GreekLetter.modifier_pattern()).unwrap(),
            greek_letters_map: HashMap::from([
                ("a", r"alpha"),
                ("b", r"beta"),
                ("g", r"gamma"),
                ("d", r"delta"),
                ("e", r"epsilon"),
                ("z", r"zeta"),
                ("n", r"eta"),
                ("th", r"theta"),
                ("i", r"iota"),
                ("k", r"kappa"),
                ("l", r"lambda"),
                ("m", r"mu"),
                ("nu", r"nu"),
                ("x", r"xi"),
                ("o", r"omicron"),
                ("p", r"pi"),
                ("r", r"rho"),
                ("s", r"sigma"),
                ("t", r"tau"),
                ("u", r"upsilon"),
                ("phi", r"phi"),
                ("chi", r"chi"),
                ("psi", r"psi"),
                ("w", r"omega"),

                ("A", r"Alpha"),
                ("B", r"Beta"),
                ("G", r"Gamma"),
                ("D", r"Delta"),
                ("E", r"Epsilon"),
                ("Z", r"Zeta"),
                ("N", r"Eta"),
                ("Th", r"Theta"),
                ("I", r"Iota"),
                ("K", r"Kappa"),
                ("L", r"Lambda"),
                ("M", r"Mu"),
                ("Nu", r"Nu"),
                ("X", r"Xi"),
                ("O", r"Omicron"),
                ("P", r"Pi"),
                ("R", r"Rho"),
                ("S", r"Sigma"),
                ("T", r"Tau"),
                ("U", r"Upsilon"),
                ("Phi", r"Phi"),
                ("Chi", r"Chi"),
                ("Psi", r"Psi"),
                ("W", r"Omega"),
            ])
        }
    }

    fn replace_with_greek_letters(&self, input: &str) -> String {
        let mut result = String::new();
        let mut i = 0;
    
        while i < input.len() {
            let mut matched = false;
            
            let mut keys: Vec<&str> = self.greek_letters_map.keys().cloned().collect();

            keys.sort_by(|a, b| b.len().cmp(&a.len()));

            for key in keys {
                if input[i..].starts_with(key) {
                    result.push_str(r"\");
                    result.push_str(self.greek_letters_map.get(key).unwrap());
                    i += key.len();
                    matched = true;
                    break;
                }
            }
    
            if !matched {
                result.push(input.chars().nth(i).unwrap());
                i += 1;
            }
        }
    
        result
    }
}

impl Debug for HtmlGreekLettersRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HtmlGreekLettersRule").field("searching_pattern", &self.search_pattern).finish()
    }
}

impl ParsingRule for HtmlGreekLettersRule {
    fn search_pattern(&self) -> &String {
        &self.search_pattern
    }

    fn standard_parse(&self, content: &str, codex: &Codex, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<ParsingOutcome, ParsingError> {
        
        let parsed_content = self.search_pattern_regex.replace_all(content, |capture: &Captures| {

            if let Some(greek_ref) = capture.get(1) {

                let res = format!(r#"<span class="greek">${}$</span>"#, self.replace_with_greek_letters(greek_ref.as_str()));

                log::debug!("parse '{}' into '{}'", content, res);

                return res;
            }

            log::error!("no greek letters found in '{}' ({})", content, capture.get(1).unwrap().as_str());

            panic!("no greek letters found");
        });

        
        Ok(ParsingOutcome::new_fixed(parsed_content.to_string()))
    }
    
    fn search_pattern_regex(&self) -> &Regex {
        &self.search_pattern_regex
    }
}
