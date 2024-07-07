use std::path::PathBuf;

use getset::{Getters, Setters};
use serde::{Deserialize, Serialize};

use crate::compiler::{parsing::parsing_configuration::list_bullet_configuration_record::{self, ListBulletConfigurationRecord}, theme::Theme};

use super::{dossier_configuration_path_reference::{DossierConfigurationPathReference, DossierConfigurationRawPathReference}, dossier_configuration_path_reference_manager::DOSSIER_CONFIGURATION_RAW_REFERENCE_MANAGER};

const STYLE_PREFIX: &str = "./assets/styles";


#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Serialize, Getters, Setters)]
pub struct DossierConfigurationStyle {

    #[serde(default)]
    #[getset(get = "pub", set = "pub")]
    theme: Theme,

    #[serde(default)]
    #[getset(get = "pub", set = "pub")]
    styles: Vec<DossierConfigurationRawPathReference>,

    #[serde(default = "default_list_bullets")]
    #[getset(get = "pub", set = "pub")]
    list_bullets_configuration: Vec<ListBulletConfigurationRecord> 
}

fn default_list_bullets() -> Vec<ListBulletConfigurationRecord> {
    list_bullet_configuration_record::default_bullets_configuration()
}

impl DossierConfigurationStyle {

    pub fn styles_references(&self) -> Vec<DossierConfigurationPathReference> {

        let dcrfm = DOSSIER_CONFIGURATION_RAW_REFERENCE_MANAGER.lock().unwrap();

        self.styles.iter().map(|raw_reference| {
            dcrfm.parse_raw_reference(raw_reference, Some(PathBuf::from(STYLE_PREFIX)))
        }).collect()
    }
}

impl Default for DossierConfigurationStyle {
    fn default() -> Self {
        Self {
            theme: Default::default(),
            styles: Default::default(),
            list_bullets_configuration: default_list_bullets()
        }
    }
}
