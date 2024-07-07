use getset::{CopyGetters, Getters, Setters};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Deserialize, Serialize, Getters, CopyGetters, Setters)]
pub struct DossierConfigurationTableOfContents {

    #[getset(get = "pub", set = "pub")]
    title: String,

    #[getset(get_copy = "pub", set = "pub")]
    include_in_output: bool,

    #[getset(get_copy = "pub", set = "pub")]
    page_numbers: bool,

    #[getset(get_copy = "pub", set = "pub")]
    plain: bool,

    #[getset(get_copy = "pub", set = "pub")]
    maximum_heading_level: usize,
}

impl Default for DossierConfigurationTableOfContents {
    fn default() -> Self {
        Self {
            title: String::from("Table of contents"),
            include_in_output: true,
            page_numbers: false,
            plain: false,
            maximum_heading_level: 4
        }
    }
}