use std::path::PathBuf;

use crate::compiler::portability_level::PortabilityLevel;

#[derive(Debug)]
pub struct AssemblerConfiguration {
    output_location: PathBuf,
    portability_level: PortabilityLevel
}

impl AssemblerConfiguration {
    
    pub fn new(output_location: PathBuf, portability_level: PortabilityLevel) -> Self {
        Self {
            output_location,
            portability_level
        }
    }

    pub fn output_location(&self) -> &PathBuf {
        &self.output_location
    }
    
    pub fn portability_level(&self) -> &PortabilityLevel {
        &self.portability_level
    }
}

impl Default for AssemblerConfiguration {
    fn default() -> Self {
        Self {
            output_location: Default::default(),
            portability_level: Default::default()
        }
    }
}