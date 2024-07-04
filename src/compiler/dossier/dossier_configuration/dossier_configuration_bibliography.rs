use std::collections::{BTreeMap, HashMap};

use getset::{CopyGetters, Getters, Setters};
use serde::{Deserialize, Serialize};

use crate::compiler::bibliography::bibliography_record::BibliographyRecord;

#[derive(Debug, Clone, Getters, CopyGetters, Setters, Deserialize, Serialize)]
pub struct DossierConfigurationBibliography {

    #[getset(get = "pub", set = "pub")]
    title: String,

    #[getset(get = "pub", set = "pub")]
    records: BTreeMap<String, BibliographyRecord>,

    #[getset(get_copy = "pub", set = "pub")]
    include_in_output: bool,
}

impl Default for DossierConfigurationBibliography {
    fn default() -> Self {
        Self {
            title: String::from("Bibliography"),
            records: Default::default(),
            include_in_output: false
        }
    }
}
