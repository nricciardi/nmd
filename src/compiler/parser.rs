

use std::{any::Any, sync::{Arc, RwLock}};


use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::compiler::{codex::modifier::standard_paragraph_modifier, parsing::parsing_outcome::ParsingOutcome};

use super::{codex::{modifier::modifiers_bucket::ModifiersBucket, Codex}, dossier::document::Paragraph, parsing::{parsing_configuration::{parsing_configuration_overlay::ParsingConfigurationOverLay, ParsingConfiguration}, parsing_error::ParsingError}};



#[derive(Debug)]
enum Segment {
    Match(String),
    NonMatch(String),
}



pub struct Parser {

}

impl Parser {

    /// Parse a text (string) using `Codex` rules and `ParsingConfiguration` options 
    pub fn parse_text(codex: &Codex, content: &str, parsing_configuration: Arc<RwLock<ParsingConfiguration>>, parsing_configuration_overlay: Arc<Option<ParsingConfigurationOverLay>>) -> Result<ParsingOutcome, ParsingError> {

        let excluded_modifiers = parsing_configuration.read().unwrap().modifiers_excluded().clone();

        Parser::parse_text_excluding_modifiers(codex, content, excluded_modifiers, Arc::clone(&parsing_configuration), parsing_configuration_overlay)
    }

    /// Parse a text (string) using `Codex` rules and `ParsingConfiguration` options, excluding a set of modifiers
    pub fn parse_text_excluding_modifiers(codex: &Codex, content: &str, mut excluded_modifiers: ModifiersBucket, 
        parsing_configuration: Arc<RwLock<ParsingConfiguration>>, parsing_configuration_overlay: Arc<Option<ParsingConfigurationOverLay>>) -> Result<ParsingOutcome, ParsingError> {

        log::debug!("start to parse content:\n{}\nexcluding: {:?}", content, excluded_modifiers);

        let mut outcome = ParsingOutcome::new(String::from(content));

        if excluded_modifiers == ModifiersBucket::All {
            log::debug!("parsing of content:\n{} is skipped are excluded all modifiers", content);
            
            return Ok(outcome)
        }

        let mut content_parts: Vec<String> = vec![String::from(content)];
        let mut content_parts_additional_excluded_modifiers: Vec<ModifiersBucket> = vec![ModifiersBucket::None];

        for text_modifier in codex.configuration().ordered_text_modifiers() {

            assert_eq!(content_parts.len(), content_parts_additional_excluded_modifiers.len());

            if excluded_modifiers.contains(text_modifier) {

                log::debug!("{:?} is skipped", text_modifier);
                continue;
            }

            let text_rule = codex.text_rules().get(text_modifier.identifier());

            if text_rule.is_none() {
                log::warn!("text rule for {:#?} not found", text_modifier);
                continue;
            }

            let text_rule = text_rule.unwrap();

            let mut new_content_parts: Vec<String> = content_parts.clone();
            let mut new_content_parts_additional_excluded_modifiers: Vec<ModifiersBucket> = content_parts_additional_excluded_modifiers.clone();
            
            for (content_part_index, content_part) in content_parts.iter().enumerate() {

                let current_excluded_modifiers = &content_parts_additional_excluded_modifiers[content_part_index];

                if current_excluded_modifiers.contains(text_modifier) {
                    log::debug!("{:?} is skipped for '{}'", text_modifier, content_part);
                }

                let matches = text_rule.find_iter(&content_part);

                if matches.len() == 0 {
                    log::debug!("'{}' => no matches with {:#?}", content_part, text_rule);
                    continue;
                }

                log::debug!("'{}' => there is a match with {:#?}", content_part, text_rule);

                let mut sub_content_parts: Vec<Segment> = Vec::new();

                let mut last_end = 0;
                for mat in matches {
                    if mat.start() > last_end {
                        sub_content_parts.push(Segment::NonMatch(content_part[last_end..mat.start()].to_string()));
                    }
                    sub_content_parts.push(Segment::Match(mat.as_str().to_string()));
                    last_end = mat.end();
                }

                if last_end < content_part.len() {
                    sub_content_parts.push(Segment::NonMatch(content_part[last_end..].to_string()));
                }

                new_content_parts.remove(content_part_index);        // remove father matched content
                new_content_parts_additional_excluded_modifiers.remove(content_part_index);


                for (sub_content_part_index, sub_content_part) in sub_content_parts.iter().enumerate() {

                    match sub_content_part {
                        Segment::Match(m) => {
                            
                            let outcome = text_rule.parse(m, codex, Arc::clone(&parsing_configuration))?;

                            new_content_parts.insert(content_part_index + sub_content_part_index, outcome.parsed_content().into());
        
                            let new_current_excluded_modifiers = current_excluded_modifiers.clone() + text_modifier.incompatible_modifiers().clone();
        
                            new_content_parts_additional_excluded_modifiers.insert(content_part_index + sub_content_part_index, new_current_excluded_modifiers);
                        

                        },
                        Segment::NonMatch(s) => {
                            new_content_parts.insert(content_part_index + sub_content_part_index, s.clone());
                            new_content_parts_additional_excluded_modifiers.insert(content_part_index + sub_content_part_index, current_excluded_modifiers.clone());


                        },
                    }

                    }                
            }

            content_parts = new_content_parts;
            content_parts_additional_excluded_modifiers = new_content_parts_additional_excluded_modifiers;
            
        }

        outcome = ParsingOutcome::new(content_parts.join(""));
        
        Ok(outcome)
    }

    /// Parse a `Paragraph` using `Codex` rules and `ParsingConfiguration` options
    pub fn parse_paragraph(codex: &Codex, paragraph: &Paragraph, parsing_configuration: Arc<RwLock<ParsingConfiguration>>, parsing_configuration_overlay: Arc<Option<ParsingConfigurationOverLay>>) -> Result<ParsingOutcome, ParsingError> {
        Parser::parse_paragraph_excluding_modifiers(codex, paragraph, ModifiersBucket::None, parsing_configuration, parsing_configuration_overlay)
    }

    /// Parse a `Paragraph` using `Codex` rules and `ParsingConfiguration` options, excluding a set of modifiers.
    /// 
    /// Only one paragraph rule can be applied on Paragraph.
    pub fn parse_paragraph_excluding_modifiers(codex: &Codex, paragraph: &Paragraph, mut excluded_modifiers: ModifiersBucket, parsing_configuration: Arc<RwLock<ParsingConfiguration>>,
        parsing_configuration_overlay: Arc<Option<ParsingConfigurationOverLay>>) -> Result<ParsingOutcome, ParsingError> {

        log::debug!("start to parse paragraph ({:?}):\n{}\nexcluding: {:?}", paragraph.paragraph_type(), paragraph, excluded_modifiers);

        let mut outcome = ParsingOutcome::new(String::from(paragraph.content()));

        if excluded_modifiers == ModifiersBucket::All {
            log::debug!("parsing of paragraph:\n{:#?} is skipped are excluded all modifiers", paragraph);
            
            return Ok(outcome)
        }

        let paragraph_modifier = codex.configuration().paragraph_modifier(paragraph.paragraph_type()).unwrap();

        let paragraph_rule = codex.paragraph_rules().get(paragraph_modifier.identifier());

        if let Some(paragraph_rule) = paragraph_rule {

            log::debug!("paragraph rule {:#?} is found, it is about to be applied to parse paragraph", paragraph_rule);

            outcome = paragraph_rule.parse(outcome.parsed_content(), codex, Arc::clone(&parsing_configuration))?;

            excluded_modifiers = excluded_modifiers + paragraph_modifier.incompatible_modifiers().clone();

        } else {

            log::warn!("there is NOT a paragraph rule for '{}' in codex", paragraph.paragraph_type());
        }
        
        outcome = Parser::parse_text_excluding_modifiers(codex, outcome.parsed_content(), excluded_modifiers, Arc::clone(&parsing_configuration), parsing_configuration_overlay)?;

        Ok(outcome)
    }
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, RwLock};

    use crate::compiler::{codex::{codex_configuration::CodexConfiguration, modifier::modifiers_bucket::ModifiersBucket, Codex}, parsing::parsing_configuration::ParsingConfiguration};

    use super::Parser;


    #[test]
    fn parse_text() {

        let codex = &Codex::of_html(CodexConfiguration::default());
        let content = "Text **bold text** `a **bold text** which must be not parsed`";
        let parsing_configuration = ParsingConfiguration::default();
        let excluded_modifiers = ModifiersBucket::None;

        let outcome = Parser::parse_text_excluding_modifiers(codex, content, excluded_modifiers, Arc::new(RwLock::new(parsing_configuration)), Arc::new(None)).unwrap();

        assert_eq!(outcome.parsed_content(), r#"Text <strong class="bold">bold text</strong> <code class="language-markup inline-code">a **bold text** which must be not parsed</code>"#)
    }
}