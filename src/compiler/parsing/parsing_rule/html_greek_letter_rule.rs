use std::{collections::HashMap, fmt::Debug, sync::{Arc, RwLock}};

use regex::{Captures, Regex};

use crate::compiler::{codex::modifier::standard_text_modifier::StandardTextModifier, parsing::{parsing_configuration::ParsingConfiguration, parsing_error::ParsingError, parsing_outcome::ParsingOutcome}};

use super::ParsingRule;


pub struct HtmlGreekLettersRule {
    searching_pattern: String,
    greek_letters_map: HashMap<&'static str, &'static str>,
}

impl HtmlGreekLettersRule {
    pub fn new() -> Self {
        Self {
            searching_pattern: StandardTextModifier::GreekLetter.modifier_pattern(),
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
}

impl Debug for HtmlGreekLettersRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HtmlGreekLettersRule").field("searching_pattern", &self.searching_pattern).finish()
    }
}

impl ParsingRule for HtmlGreekLettersRule {
    fn searching_pattern(&self) -> &String {
        &self.searching_pattern
    }

    fn parse(&self, content: &str, parsing_configuration: Arc<RwLock<ParsingConfiguration>>) -> Result<ParsingOutcome, ParsingError> {
        
        let regex = Regex::new(&self.searching_pattern).unwrap();

        let parsed_content = regex.replace_all(content, |capture: &Captures| {

            if let Some(greek_ref) = capture.get(1) {
                if let Some(value) = self.greek_letters_map.get(greek_ref.as_str()) {

                    let res = format!(r#"<span class="greek">$\{}$</span>"#, value);

                    log::debug!("parse '{}' into '{}'", content, res);

                    return res;
                }
            }

            log::warn!("no greek letters found in '{}'", content);

            capture.get(1).unwrap().as_str().to_string()
        });

        
        Ok(ParsingOutcome::new(parsed_content.to_string()))
    }
}
