pub mod bibliography_entry;

use std::{collections::{BTreeMap, HashMap}, sync::{Arc, RwLock}};

use bibliography_entry::BibliographyEntry;
use getset::{Getters, Setters};

use crate::resource::resource_reference::ResourceReference;

use super::{codex::Codex, dossier::{self, document::chapter::heading::Heading, Dossier}, output_format::OutputFormat, parsable::Parsable, parser::Parser, parsing::{parsing_configuration::{parsing_configuration_overlay::ParsingConfigurationOverLay, ParsingConfiguration}, parsing_error::ParsingError, parsing_outcome::ParsingOutcome}};



#[derive(Debug, Clone, Getters, Setters)]
pub struct Bibliography {

    #[getset(get = "pub", set = "pub")]
    title: String,

    #[getset(get = "pub", set = "pub")]
    entries: BTreeMap<String, BibliographyEntry>,

    #[getset(get = "pub", set = "pub")]
    parsed_content: Option<ParsingOutcome>,
}

impl Bibliography {
    pub fn new(title: String, entries: BTreeMap<String, BibliographyEntry>) -> Self {
        Self {
            title,
            entries,
            parsed_content: None,
        }
    }
}

impl Parsable for Bibliography {
    fn standard_parse(&mut self, format: &OutputFormat, codex: Arc<Codex>, parsing_configuration: Arc<RwLock<ParsingConfiguration>>, parsing_configuration_overlay: Arc<Option<ParsingConfigurationOverLay>>) -> Result<(), ParsingError> {
        
        let mut outcome = ParsingOutcome::new_empty();

        outcome.add_fixed_part(String::from(r#"<section class="bibliography">"#));
        outcome.add_fixed_part(String::from(r#"<span class="bibliography-title">"#));
        outcome.append_parsing_outcome(&mut Parser::parse_text(&codex, &self.title, Arc::clone(&parsing_configuration), Arc::clone(&parsing_configuration_overlay))?);
        outcome.add_fixed_part(String::from(r#"</span>"#));
        outcome.add_fixed_part(String::from(r#"<ul class="bibliography-body">"#));


        // for heading in &self.headings {

        //     let heading_lv: usize = heading.level() as usize;

        //     if heading_lv > self.maximum_heading_level {
        //         continue;
        //     }

        //     outcome.add_fixed_part(String::from(r#"<li class="bibliography-item">"#));

        //     outcome.add_fixed_part(r#"<span class="bibliography-item-bullet">"#.to_string());
        //     outcome.add_fixed_part(r#"</span><div class="bibliography-item-content">"#.to_string());
            
        //     // TODO: numero, link

        //     outcome.add_fixed_part(String::from(r#"</div></li>"#));

        //     total_li += 1;
                
        // }

        outcome.add_fixed_part(String::from(r#"</ul>"#));
        outcome.add_fixed_part(String::from(r#"</section>"#));

        self.parsed_content = Some(outcome);

        Ok(())
    }
}