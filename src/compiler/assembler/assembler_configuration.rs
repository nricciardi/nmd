use std::{ffi::OsStr, path::PathBuf};

use crate::compiler::{dossier::dossier_configuration::{self, DossierConfiguration}, theme::Theme};

#[derive(Debug)]
pub struct AssemblerConfiguration {
    output_location: PathBuf,
    theme: Theme,
    use_remote_addons: bool,

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

    pub fn output_location(&self) -> &PathBuf {
        &self.output_location
    }

    pub fn output_dir_location(&self) -> PathBuf {
        if self.output_location.is_dir() {
            return self.output_location.clone()
        }

        self.output_location.parent().unwrap().into()
    }

    pub fn output_filename_location(&self) -> Option<&OsStr> {
        if self.output_location.is_file() {
            return self.output_location.file_name()
        }

        None
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }
    
    pub fn use_remote_addons(&self) -> bool {
        self.use_remote_addons
    }

    pub fn parallelization(&self) -> bool {
        self.parallelization
    }

    pub fn set_output_location(&mut self, value: PathBuf) {
        self.output_location = value
    }

    pub fn set_theme(&mut self, value: Theme) {
        self.theme = value
    }

    pub fn set_use_remote_addons(&mut self, value: bool) {
        self.use_remote_addons = value
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