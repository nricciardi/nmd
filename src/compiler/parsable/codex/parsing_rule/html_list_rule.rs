use std::sync::Arc;

use regex::Regex;

use crate::compiler::parsable::{codex::{modifier::Mod, Modifier}, parsing_configuration::list_bullet_configuration_record::{self, ListBulletConfigurationRecord}, ParsingConfiguration};
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

    fn transform_to_field(to: String) -> String {

        if to.eq(list_bullet_configuration_record::CHECKBOX) {
            return String::from(r#"<div class="checkbox checkbox-unchecked"></div>"#)
        }

        if to.eq(list_bullet_configuration_record::CHECKBOX_CHECKED) {
            return String::from(r#"<div class="checkbox checkbox-checked"></div>"#)
        }

        to
    }

    fn bullet_transform(bullet: &str, indentation_level: usize, list_bullets_configurations: &Vec<ListBulletConfigurationRecord>) -> String {

        for bullet_configuration in list_bullets_configurations {

            if bullet_configuration.from.eq(bullet) {
                if bullet_configuration.strict_indentation && indentation_level == bullet_configuration.indentation_level {

                    return Self::transform_to_field(bullet_configuration.to.clone())

                } else if !bullet_configuration.strict_indentation && indentation_level >= bullet_configuration.indentation_level {
                    return Self::transform_to_field(bullet_configuration.to.clone())
                }
            }
        }

        String::from(bullet)
    }
}

impl ParsingRule for HtmlListRule {
    fn modifier(&self) -> &dyn Mod {
        &Modifier::List
    }

    fn parse(&self, content: &str, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError> {
        
        let mut parsed_content = String::new();

        parsed_content.push_str(r#"<ul class="list">"#);

        let search_patter = Modifier::ListItem.search_pattern();

        let regex = Regex::new(&search_patter).unwrap();

        let mut items_found = 0;

        for captures in regex.captures_iter(content) {
            if let Some(indentation) = captures.get(1) {
                if let Some(bullet) = captures.get(2) {
                    if let Some(content) = captures.get(3) {

                        items_found += 1;

                        let mut indentation = String::from(indentation.as_str());
                        let bullet = bullet.as_str();
                        let content = content.as_str();
                        
                        indentation = indentation.replace("\t", SPACE_TAB_EQUIVALENCE);

                        let mut indentation_level: usize = 0;
                        while indentation.starts_with(SPACE_TAB_EQUIVALENCE) {
                            indentation = indentation.split_off(SPACE_TAB_EQUIVALENCE.len());
                            indentation_level += 1;
                        }

                        let bullet = Self::bullet_transform(bullet, indentation_level, parsing_configuration.list_bullets_configuration());

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

        let total_valid_lines = content.lines().into_iter().filter(|l| !l.is_empty() && !l.to_string().eq("\n")).count();

        if items_found != total_valid_lines {

            if parsing_configuration.strict_list_check() {
                log::error!("the following list has incorrect items (parsed {} on {}):\n{}", items_found, total_valid_lines, content);
                panic!("incorrect list item(s)")
            } else {
                log::warn!("the following list has incorrect items (parsed {} on {}):\n{}\n", items_found, total_valid_lines, content);
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
        | element 2.1.1b
        - element 2.1.2
        - element 2.1.3
    - element 2.2
- element 3
"#.trim();
       
       let rule = HtmlListRule::new();

       let regex = Regex::new(&rule.modifier().search_pattern()).unwrap();

       let outcome = rule.parse(nmd_text, Arc::new(ParsingConfiguration::default())).unwrap();

    }
}