use std::sync::Arc;

use regex::Regex;

use crate::compiler::parsable::{codex::Modifier, ParsingConfiguration};
use super::{parsing_outcome::{ParsingError, ParsingOutcome}, ParsingRule};

const SPACE_TAB_EQUIVALENCE: &str = r"   ";
const INDENTATION: &str = r#"<span class="list-item-indentation"></span>"#;


pub struct HtmlListRule {

}

impl HtmlListRule {
    pub fn new() -> Self {
        Self {
            
        }
    }
}

impl HtmlListRule {
    fn bullet_transform(bullet: &str, indentation_level: usize) -> String {

        // TODO: based on parsing configuration

        String::from(match bullet {
            "-" if indentation_level == 0 => r"&bull;",
            "-" if indentation_level == 1 => r"&#9702;",
            "-" if indentation_level >= 2 => r"&#8211;",
            "*" => r"&#9654;",
            _ => bullet
        })
    }
}

impl ParsingRule for HtmlListRule {
    fn modifier(&self) -> &Modifier {
        &Modifier::List
    }

    fn parse(&self, content: &str, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError> {
        
        let mut parsed_content = String::new();

        parsed_content.push_str(r#"<ul class="list">"#);

        let search_patter = self.modifier().search_pattern().replace(r"(?s)", r"(?m)^");

        let regex = Regex::new(&search_patter).unwrap();

        for captures in regex.captures_iter(content) {
            if let Some(indentation) = captures.get(1) {
                if let Some(bullet) = captures.get(2) {
                    if let Some(content) = captures.get(3) {

                        let mut indentation = String::from(indentation.as_str());
                        let bullet = bullet.as_str();
                        let content = content.as_str();
                        
                        indentation = indentation.replace("\t", SPACE_TAB_EQUIVALENCE);

                        let mut indentation_level: usize = 0;
                        while indentation.starts_with(SPACE_TAB_EQUIVALENCE) {
                            indentation = indentation.split_off(SPACE_TAB_EQUIVALENCE.len());
                            indentation_level += 1;
                        }

                        let bullet = Self::bullet_transform(bullet, indentation_level);

                        parsed_content.push_str(format!(r#"
                        <li class="list-item">
                            {}
                            <span class="list-item-bullet">
                                {}
                            </span>
                            <span class="list-item-content">
                                {}
                            </span>
                        </li>"#, INDENTATION.repeat(indentation_level), bullet, content).trim());

                    }
                }
            }
        }

        parsed_content.push_str("</ul>");
        
        Ok(ParsingOutcome::new(parsed_content))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing() {
        let nmd_text = r#"
- element 1
- element 2
    - element 2.1
        - element 2.1.1a
        element 2.1.1b
        - element 2.1.2
        - element 2.1.3
    - element 2.2
"#.trim();
       
       let rule = HtmlListRule::new();

       let regex = Regex::new(&rule.modifier().search_pattern()).unwrap();

       let outcome = rule.parse(nmd_text, Arc::new(ParsingConfiguration::default())).unwrap();

    }
}