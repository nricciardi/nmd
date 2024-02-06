use serde::{Deserialize, Serialize};

use crate::compiler::theme::Theme;


#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize, Default, Serialize)]
pub struct DossierConfigurationStyle {
    theme: Theme,
    addons: Vec<String>
}

impl DossierConfigurationStyle {
    pub fn theme(&self) -> &Theme {
        &self.theme
    }
}
