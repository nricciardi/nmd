pub mod parsing_rule;
pub mod parsable;
pub mod parsed_content_accessor;

use std::sync::Arc;


use self::parsing_rule::{parsing_configuration::ParsingConfiguration, parsing_error::ParsingError, parsing_outcome::ParsingOutcome};

use super::{codex::{modifier::modifiers_bucket::ModifiersBucket, Codex}, dossier::document::{Chapter, Paragraph}};



pub struct Parser {

}

impl Parser {


    pub fn parse_text(codex: &Codex, content: &str, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError> {

        let excluded_modifiers = parsing_configuration.modifiers_excluded().clone();

        Parser::parse_text_excluding_modifiers(codex, content, Arc::clone(&parsing_configuration), excluded_modifiers)
    }

    pub fn parse_text_excluding_modifiers(codex: &Codex, content: &str, parsing_configuration: Arc<ParsingConfiguration>, mut excluded_modifiers: ModifiersBucket) -> Result<ParsingOutcome, ParsingError> {

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
    
                excluded_modifiers = excluded_modifiers + text_rule.incompatible_modifiers().clone();

                if excluded_modifiers == ModifiersBucket::All {
                    log::debug!("all next modifiers will be skipped because {:?} is found", ModifiersBucket::All);
                    break;
                }

            } else {
                log::debug!("no matches with {:#?}", text_rule);
            }
            
        }

        Ok(outcome)
    }

    pub fn parse_paragraph(codex: &Codex, paragraph: &Paragraph, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError> {
        Parser::parse_paragraph_excluding_modifiers(codex, paragraph, parsing_configuration, ModifiersBucket::None)
    }

    pub fn parse_paragraph_excluding_modifiers(codex: &Codex, paragraph: &Paragraph, parsing_configuration: Arc<ParsingConfiguration>, mut excluded_modifiers: ModifiersBucket) -> Result<ParsingOutcome, ParsingError> {

        log::debug!("start to parse paragraph:\n'{}'\nexcluding: {:?}", paragraph, excluded_modifiers);

        let mut outcome = ParsingOutcome::new(String::from(paragraph.content()));

        if excluded_modifiers == ModifiersBucket::All {
            log::debug!("parsing of paragraph:\n{} is skipped are excluded all modifiers", paragraph);
            
            return Ok(outcome)
        }

        let paragraph_rule = codex.paragraph_rules().get(paragraph.paragraph_type());

        if let Some(paragraph_rule) = paragraph_rule {
            let search_pattern = paragraph_rule.search_pattern();

            log::debug!("'{}' paragraph search pattern that is about to be tested", search_pattern);

            outcome = paragraph_rule.parse(outcome.parsed_content(), Arc::clone(&parsing_configuration))?;

            excluded_modifiers = excluded_modifiers + paragraph_rule.incompatible_modifiers().clone();

        } else {

            log::warn!("there is NOT a paragraph rule for '{}' in codex", paragraph.paragraph_type());
        }

        // for paragraph_rule in self.paragraph_rules() {

        //     let search_pattern = paragraph_rule.modifier().search_pattern();

        //     log::debug!("{:?}: '{}' paragraph search pattern that is about to be tested", paragraph_rule.modifier(), search_pattern);

        //     if paragraph_rule.is_match(outcome.parsed_content()) {

        //         log::debug!("there is a match with '{}'", search_pattern);

        //         outcome = paragraph_rule.parse(outcome.parsed_content(), Arc::clone(&parsing_configuration))?;

        //         excluded_modifiers = excluded_modifiers + paragraph_rule.incompatible_modifiers().clone();

        //         break;      // ONLY ONE paragraph modifier
        //     } else {

        //         log::debug!("there is NOT a match with '{}'", search_pattern);
        //     }
        // }

        outcome = Parser::parse_text_excluding_modifiers(codex, outcome.parsed_content(), Arc::clone(&parsing_configuration), excluded_modifiers)?;

        Ok(outcome)
    }

    pub fn parse_chapter(codex: &Codex, chapter: &Chapter, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError> {
        // TODO

        let parsing_outcome = ParsingOutcome::new(codex.parse_text(&self.heading, Arc::clone(&parsing_configuration))?.parsed_content());


        log::debug!("parsing chapter:\n{:#?}", self);

        if parsing_configuration.parallelization() {

            //! TODO: not in order!!!!!

            let maybe_failed = self.paragraphs.par_iter_mut()
                .map(|paragraph| {
                    let result = paragraph.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration));

                    if let Ok(result) = result {
                        parsing_outcome.append_parsed_content(&result.parsed_content())
                    }

                    result.map(|r| ())
                })
                .find_any(|result| result.is_err());
    
            if let Some(result) = maybe_failed {
                return Err(result.err().unwrap());
            }

        } else {
            
            let maybe_failed = self.paragraphs.iter_mut()
                .map(|paragraph| {
                    let result = paragraph.parse(Arc::clone(&codex), Arc::clone(&parsing_configuration));

                    if let Ok(result) = result {
                        parsing_outcome.append_parsed_content(&result.parsed_content())
                    }

                    result.map(|r| ())
                })
                .find(|result| result.is_err());
    
            if let Some(result) = maybe_failed {
                return Err(result.err().unwrap());
            }
        }

        Ok(parsing_outcome)
    }
}