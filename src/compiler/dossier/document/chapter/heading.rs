use std::{num::ParseIntError, str::FromStr, sync::{Arc, RwLock}};

use crate::{compiler::{codex::Codex, output_format::OutputFormat, parsable::{parsed_content_accessor::ParsedContentAccessor, Parsable}, parser::Parser, parsing::{parsing_configuration::{parsing_configuration_overlay::ParsingConfigurationOverLay, ParsingConfiguration}, parsing_error::ParsingError, parsing_metadata::ParsingMetadata, parsing_outcome::{ParsingOutcome, ParsingOutcomePart}}}, resource::resource_reference::ResourceReference};

use super::chapter_builder::ChapterBuilderError;


pub type HeadingLevel = u32;


#[derive(Debug, Clone)]
pub struct Heading {
    level: HeadingLevel,
    title: String,

    // raw_content: String,
    parsed_content: Option<ParsingOutcome>
}

impl Heading {
    pub fn new(level: HeadingLevel, title: String) -> Self {

        Self {
            level,
            title,
            parsed_content: None
        }
    }

    pub fn level(&self) -> HeadingLevel {
        self.level
    }

    pub fn title(&self) -> &String {
        &self.title
    }
}

impl Parsable for Heading {
    fn standard_parse(&mut self, format: &OutputFormat, codex: Arc<Codex>, parsing_configuration: Arc<RwLock<ParsingConfiguration>>, parsing_configuration_overlay: Arc<Option<ParsingConfigurationOverLay>>) -> Result<(), ParsingError> {

        let pc = parsing_configuration.read().unwrap();

        let document_name = pc.metadata().document_name().as_ref().unwrap();

        let id = ResourceReference::of_internal_without_sharp(&self.title, Some(&document_name))?;

        let parsed_title = Parser::parse_text(&codex, &self.title, Arc::clone(&parsing_configuration), parsing_configuration_overlay)?;

        let outcome = ParsingOutcome::new(vec![
            ParsingOutcomePart::Fixed { content: format!(r#"<h{} class="heading-{}" id="{}">"#, self.level, self.level, id.build_without_internal_sharp()) },
            ParsingOutcomePart::Mutable { content: parsed_title.parsed_content() },
            ParsingOutcomePart::Fixed { content: format!(r#"</h{}>"#, self.level) },
        ]);
        
        self.parsed_content = Some(outcome);

        Ok(())
    }
}


impl ParsedContentAccessor for Heading {
    fn parsed_content(&self) -> &Option<ParsingOutcome> {
        &self.parsed_content
    }
}
