use getset::{CopyGetters, Getters, Setters};
use serde::{Deserialize, Serialize};

pub const CHECKBOX: &str = ":checkbox:";
pub const CHECKBOX_CHECKED: &str = ":checkbox-checked:";


#[derive(Debug, Clone, Deserialize, Serialize, Getters, CopyGetters, Setters)]
pub struct ListBulletConfigurationRecord {

    #[getset(get = "pub", set = "pub")]
    pub from: String,

    #[getset(get = "pub", set = "pub")]
    pub to: String,

    #[getset(get_copy = "pub", set = "pub")]
    pub indentation_level: usize,

    #[getset(get_copy = "pub", set = "pub")]
    pub strict_indentation: bool
}

impl ListBulletConfigurationRecord {

    pub fn new(from: String, to: String, indentation_level: usize, strict_indentation: bool) -> Self {
        Self {
            from,
            to,
            indentation_level,
            strict_indentation
        }
    }
}

pub fn default_bullets_configuration() -> Vec<ListBulletConfigurationRecord> {
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
        ListBulletConfigurationRecord {
            from: String::from(r"-[ ]"),
            to: String::from(r":checkbox:"),
            indentation_level: 0,
            strict_indentation: false
        },
        ListBulletConfigurationRecord {
            from: String::from(r"-[x]"),
            to: String::from(r":checkbox-checked:"),
            indentation_level: 0,
            strict_indentation: false
        },
        ListBulletConfigurationRecord {
            from: String::from(r"-[X]"),
            to: String::from(r":checkbox-checked:"),
            indentation_level: 0,
            strict_indentation: false
        },
    ]
}