mod content_tree;

use std::sync::{Arc, RwLock};

use content_tree::ContentTree;

use super::{codex::Codex, output_format::OutputFormat, parsable::Parsable, parsing::{parsing_configuration::{parsing_configuration_overlay::ParsingConfigurationOverLay, ParsingConfiguration}, parsing_error::ParsingError}};




#[derive(Debug, Clone)]
pub struct TableOfContents {
    title: String,
    page_numbers: bool,
    tabulated: bool,
    maximum_heading_level: usize,
    content: ContentTree,
}

impl TableOfContents {
    pub fn new(title: String, page_numbers: bool, tabulated: bool, maximum_heading_level: usize, content: ContentTree) -> Self {
        Self {
            title,
            page_numbers,
            tabulated,
            maximum_heading_level,
            content
        }
    }
}

impl Parsable for TableOfContents {
    fn standard_parse(&mut self, format: &OutputFormat, codex: Arc<Codex>, parsing_configuration: Arc<RwLock<ParsingConfiguration>>, parsing_configuration_overlay: Arc<Option<ParsingConfigurationOverLay>>) -> Result<(), ParsingError> {
        

        
        todo!()
    }
}