use serde::{Deserialize, Serialize};

use crate::compiler::{parsable::parsing_configuration::list_bullet_configuration_record::ListBulletConfigurationRecord, theme::Theme};


#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DossierConfigurationStyle {

    #[serde(default)]
    theme: Theme,

    #[serde(default)]
    addons: Vec<String>,

    #[serde(default = "default_list_bullets")]
    list_bullets_configuration: Vec<ListBulletConfigurationRecord> 
}

fn default_list_bullets() -> Vec<ListBulletConfigurationRecord> {
    vec![
        ListBulletConfigurationRecord {
            from: String::from(r"|"),
            to: String::from(r"&#8205;"),
            indentation_level: 0,
            strict_indentation: false
        },
        ListBulletConfigurationRecord {
            from: String::from(r"-"),
            to: String::from(r"&bull;"),
            indentation_level: 0,
            strict_indentation: true
        },
        ListBulletConfigurationRecord {
            from: String::from(r"-"),
            to: String::from(r"&#9702;"),
            indentation_level: 1,
            strict_indentation: true
        },
        ListBulletConfigurationRecord {
            from: String::from(r"-"),
            to: String::from(r"&#8211;"),
            indentation_level: 2,
            strict_indentation: false
        },
        ListBulletConfigurationRecord {
            from: String::from(r"*"),
            to: String::from(r"&bull;"),
            indentation_level: 0,
            strict_indentation: false
        },
        ListBulletConfigurationRecord {
            from: String::from(r"+"),
            to: String::from(r"&#9702;"),
            indentation_level: 0,
            strict_indentation: false
        },
        ListBulletConfigurationRecord {
            from: String::from(r"->"),
            to: String::from(r"&#9654;"),
            indentation_level: 0,
            strict_indentation: false
        },
        ListBulletConfigurationRecord {
            from: String::from(r"--"),
            to: String::from(r"&#8211;"),
            indentation_level: 0,
            strict_indentation: false
        },
        ListBulletConfigurationRecord {
            from: String::from(r"-[]"),
            to: String::from(r":checkbox:"),
            indentation_level: 0,
            strict_indentation: false
        },
    ]
}

impl DossierConfigurationStyle {
    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    pub fn list_bullets_configuration(&self) -> &Vec<ListBulletConfigurationRecord> {
        &self.list_bullets_configuration
    }
}

impl Default for DossierConfigurationStyle {
    fn default() -> Self {
        Self {
            theme: Default::default(),
            addons: Default::default(),
            list_bullets_configuration: default_list_bullets()
        }
    }
}
