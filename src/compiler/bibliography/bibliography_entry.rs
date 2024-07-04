use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BibliographyEntry {
    title: String,
    year: Option<u32>,
    authors: Option<Vec<String>>,
    description: Option<String>,
    url: Option<String>,
}

impl BibliographyEntry {

}