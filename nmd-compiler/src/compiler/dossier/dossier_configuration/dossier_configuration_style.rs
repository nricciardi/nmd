use serde::Deserialize;

use crate::compiler::theme::Theme;


#[derive(Debug, Clone, Deserialize, Default)]
pub struct DossierConfigurationStyle {
    theme: Theme,
    addons: Vec<String>
}

impl DossierConfigurationStyle {
    pub fn theme(&self) -> &Theme {
        &self.theme
    }
}
