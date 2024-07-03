use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DossierConfigurationTableOfContents {
    title: String,
    include_in_output: bool,
    page_numbers: bool,
    tabulated: bool,
    maximum_heading_level: usize,
}

impl Default for DossierConfigurationTableOfContents {
    fn default() -> Self {
        Self {
            title: String::from("Table of contents"),
            include_in_output: false,
            page_numbers: false,
            tabulated: true,
            maximum_heading_level: 4
        }
    }
}