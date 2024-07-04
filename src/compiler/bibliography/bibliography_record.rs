use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Getters, Setters)]
pub struct BibliographyRecord {

    #[getset(get = "pub", set = "pub")]
    title: String,

    #[getset(get = "pub", set = "pub")]
    year: Option<u32>,

    #[getset(get = "pub", set = "pub")]
    authors: Option<Vec<String>>,

    #[getset(get = "pub", set = "pub")]
    description: Option<String>,
    
    #[getset(get = "pub", set = "pub")]
    url: Option<String>,
}

impl BibliographyRecord {

}