use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DossierConfigurationTableOfContents {
    title: String,
    include_in_output: bool,
    page_numbers: bool,
    plain: bool,
    maximum_heading_level: usize,
}

impl DossierConfigurationTableOfContents {
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn page_numbers(&self) -> bool {
        self.page_numbers
    }

    pub fn maximum_heading_level(&self) -> usize {
        self.maximum_heading_level
    }

    pub fn include_in_output(&self) -> bool {
        self.include_in_output
    }

    pub fn plain(&self) -> bool {
        self.plain
    }
}

impl Default for DossierConfigurationTableOfContents {
    fn default() -> Self {
        Self {
            title: String::from("Table of contents"),
            include_in_output: false,
            page_numbers: false,
            plain: false,
            maximum_heading_level: 4
        }
    }
}