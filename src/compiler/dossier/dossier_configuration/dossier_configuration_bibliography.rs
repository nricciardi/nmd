use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

use crate::compiler::bibliography::bibliography_entry::BibliographyEntry;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DossierConfigurationBibliography {
    title: String,
    entries: BTreeMap<String, BibliographyEntry>,
}

impl Default for DossierConfigurationBibliography {
    fn default() -> Self {
        Self {
            title: String::from("Bibliography"),
            entries: Default::default()
        }
    }
}
