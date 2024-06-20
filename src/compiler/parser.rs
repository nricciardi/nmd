

use std::{any::Any, sync::{Arc, RwLock}};


use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::compiler::{codex::modifier::standard_paragraph_modifier, parsing::parsing_outcome::ParsingOutcome};

use super::{codex::{modifier::modifiers_bucket::ModifiersBucket, Codex}, dossier::document::Paragraph, parsing::{parsing_configuration::{parsing_configuration_overlay::ParsingConfigurationOverLay, ParsingConfiguration}, parsing_error::ParsingError}};



pub struct Parser {

}

impl Parser {


    pub fn parse_text(codex: &Codex, content: &str, parsing_configuration: Arc<RwLock<ParsingConfiguration>>, parsing_configuration_overlay: Arc<Option<ParsingConfigurationOverLay>>) -> Result<ParsingOutcome, ParsingError> {

        let excluded_modifiers = parsing_configuration.read().unwrap().modifiers_excluded().clone();

        Parser::parse_text_excluding_modifiers(codex, content, Arc::clone(&parsing_configuration), excluded_modifiers, parsing_configuration_overlay)
    }

    pub fn parse_text_excluding_modifiers(codex: &Codex, content: &str, parsing_configuration: Arc<RwLock<ParsingConfiguration>>,
        mut excluded_modifiers: ModifiersBucket, parsing_configuration_overlay: Arc<Option<ParsingConfigurationOverLay>>) -> Result<ParsingOutcome, ParsingError> {

        log::debug!("start to parse content:\n{}\nexcluding: {:?}", content, excluded_modifiers);

        let mut outcome = ParsingOutcome::new(String::from(content));

        if excluded_modifiers == ModifiersBucket::All {
            log::debug!("parsing of content:\n{} is skipped are excluded all modifiers", content);
            
            return Ok(outcome)
        }

        for text_modifier in codex.configuration().ordered_text_modifiers() {

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

            if text_rule.is_match(content) {
                log::debug!("there is a match with {:#?}", text_rule);

                outcome = text_rule.parse(outcome.parsed_content(), Arc::clone(&parsing_configuration))?;
    
                excluded_modifiers = excluded_modifiers + text_modifier.incompatible_modifiers().clone();

                if excluded_modifiers == ModifiersBucket::All {
                    log::debug!("all next modifiers will be skipped because {:?} is found", ModifiersBucket::All);
                    break;
                }

            } else {
                log::debug!("no matches with {:?}", text_rule);
            }
            
        }

        Ok(outcome)
    }

    pub fn parse_paragraph(codex: &Codex, paragraph: &Paragraph, parsing_configuration: Arc<RwLock<ParsingConfiguration>>, parsing_configuration_overlay: Arc<Option<ParsingConfigurationOverLay>>) -> Result<ParsingOutcome, ParsingError> {
        Parser::parse_paragraph_excluding_modifiers(codex, paragraph, parsing_configuration, ModifiersBucket::None, parsing_configuration_overlay)
    }

    pub fn parse_paragraph_excluding_modifiers(codex: &Codex, paragraph: &Paragraph, parsing_configuration: Arc<RwLock<ParsingConfiguration>>,
        mut excluded_modifiers: ModifiersBucket, parsing_configuration_overlay: Arc<Option<ParsingConfigurationOverLay>>) -> Result<ParsingOutcome, ParsingError> {

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

            outcome = paragraph_rule.parse(outcome.parsed_content(), Arc::clone(&parsing_configuration))?;

            excluded_modifiers = excluded_modifiers + paragraph_modifier.incompatible_modifiers().clone();

        } else {

            log::warn!("there is NOT a paragraph rule for '{}' in codex", paragraph.paragraph_type());
        }
        
        outcome = Parser::parse_text_excluding_modifiers(codex, outcome.parsed_content(), Arc::clone(&parsing_configuration), excluded_modifiers, parsing_configuration_overlay)?;

        Ok(outcome)
    }
}