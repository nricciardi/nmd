use serde::{Deserialize, Serialize};


pub type DossierConfigurationBibliography = Vec<DossierConfigurationBibliographyEntry>;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DossierConfigurationBibliographyEntry {
    title: String,
    year: Option<u32>,
    authors: Option<Vec<String>>,
    description: Option<String>
}

impl DossierConfigurationBibliographyEntry {

}