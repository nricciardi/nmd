use serde::{Deserialize, Serialize};

use crate::compiler::{parsing::parsing_configuration::list_bullet_configuration_record::{self, ListBulletConfigurationRecord}, theme::Theme};

use super::dossier_configuration_raw_reference::DossierConfigurationRawReference;



#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DossierConfigurationStyle {

    #[serde(default)]
    theme: Theme,

    #[serde(default)]
    styles: Vec<DossierConfigurationRawReference>,

    #[serde(default = "default_list_bullets")]
    list_bullets_configuration: Vec<ListBulletConfigurationRecord> 
}

fn default_list_bullets() -> Vec<ListBulletConfigurationRecord> {
    list_bullet_configuration_record::default_bullets_configuration()
}

impl DossierConfigurationStyle {
    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    pub fn list_bullets_configuration(&self) -> &Vec<ListBulletConfigurationRecord> {
        &self.list_bullets_configuration
    }

    pub fn styles_references(&self) -> &Vec<String> {
        &self.styles
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
