use getset::{CopyGetters, Getters, Setters};

use crate::compiler::{dossier::dossier_configuration::DossierConfiguration, theme::Theme};

#[derive(Debug, Getters, CopyGetters, Setters)]
pub struct AssemblerConfiguration {

    #[getset(get = "pub", set = "pub")]
    theme: Theme,

    #[getset(get_copy = "pub", set = "pub")]
    use_remote_addons: bool,

    #[getset(get_copy = "pub", set = "pub")]
    parallelization: bool,

    #[getset(get = "pub", set = "pub")]
    styles_raw_path: Vec<String>,

    #[getset(get_copy = "pub", set = "pub")]
    preview: bool,

    #[getset(get_copy = "pub", set = "pub")]
    watching: bool,
}

impl AssemblerConfiguration {
    
    pub fn new(theme: Theme, use_remote_addons: bool, parallelization: bool, preview: bool, watching: bool,) -> Self {
        Self {
            theme,
            use_remote_addons,
            parallelization,
            styles_raw_path: Vec::new(),
            preview,
            watching,
        }
    }
}

impl Default for AssemblerConfiguration {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            use_remote_addons: false,
            parallelization: false,
            styles_raw_path: Vec::new(),
            preview: false,
            watching: false,
        }
    }
}

impl From<DossierConfiguration> for AssemblerConfiguration {
    fn from(dossier_configuration: DossierConfiguration) -> Self {
        Self {
            theme: dossier_configuration.style().theme().clone(),
            use_remote_addons: dossier_configuration.compilation().use_remote_addons(),
            parallelization: dossier_configuration.compilation().parallelization(),

            ..Default::default()
        }
    }
}