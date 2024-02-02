use std::path::PathBuf;

use crate::compiler::theme::Theme;

#[derive(Debug)]
pub struct AssemblerConfiguration {
    output_location: PathBuf,
    theme: Theme
}

impl AssemblerConfiguration {
    
    pub fn new(output_location: PathBuf, theme: Theme) -> Self {
        Self {
            output_location,
            theme
        }
    }

    pub fn output_location(&self) -> &PathBuf {
        &self.output_location
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }
    
}

impl Default for AssemblerConfiguration {
    fn default() -> Self {
        Self {
            output_location: Default::default(),
            theme: Theme::default()
        }
    }
}