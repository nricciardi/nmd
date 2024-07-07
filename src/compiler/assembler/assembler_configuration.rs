use std::{ffi::OsStr, path::PathBuf};

use getset::{CopyGetters, Getters, Setters};

use crate::compiler::{dossier::dossier_configuration::{self, DossierConfiguration}, theme::Theme};

#[derive(Debug, Getters, CopyGetters, Setters)]
pub struct AssemblerConfiguration {

    #[getset(get = "pub", set = "pub")]
    output_location: PathBuf,

    #[getset(get = "pub", set = "pub")]
    theme: Theme,

    #[getset(get_copy = "pub", set = "pub")]
    use_remote_addons: bool,

    #[getset(get_copy = "pub", set = "pub")]
    parallelization: bool,

}

impl AssemblerConfiguration {
    
    pub fn new(output_location: PathBuf, theme: Theme, use_remote_addons: bool, parallelization: bool) -> Self {
        Self {
            output_location,
            theme,
            use_remote_addons,
            parallelization
        }
    }
}

impl Default for AssemblerConfiguration {
    fn default() -> Self {
        Self {
            output_location: Default::default(),
            theme: Theme::default(),
            use_remote_addons: false,
            parallelization: false
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