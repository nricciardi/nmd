use std::path::PathBuf;

#[derive(Debug)]
pub struct AssemblerConfiguration {
    output_location: PathBuf
}

impl AssemblerConfiguration {
    
    pub fn new(output_location: PathBuf) -> Self {
        Self {
            output_location
        }
    }

    pub fn output_location(&self) -> &PathBuf {
        &self.output_location
    }
    
}

impl Default for AssemblerConfiguration {
    fn default() -> Self {
        Self {
            output_location: Default::default()
        }
    }
}