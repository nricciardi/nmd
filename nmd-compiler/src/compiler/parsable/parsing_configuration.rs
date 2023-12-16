use crate::compiler::portability_level::PortabilityLevel;




#[derive(Clone, Default)]
pub struct ParsingConfigurationMetadata {}

pub struct ParsingConfiguration {
    metadata: ParsingConfigurationMetadata,
    portability_level: PortabilityLevel
}

impl ParsingConfiguration {

    pub fn metadata(&self) -> &ParsingConfigurationMetadata {
        &self.metadata
    }

    pub fn portability_level(&self) -> &PortabilityLevel {
        &self.portability_level
    }

    pub fn new(metadata: ParsingConfigurationMetadata, portability_level: PortabilityLevel) -> Self {
        Self {
            metadata,
            portability_level
        }
    }
}

impl Default for ParsingConfiguration {
    fn default() -> Self {
        ParsingConfiguration {
            metadata: ParsingConfigurationMetadata {  },
            portability_level: PortabilityLevel::AllInOne
        }
    }
}

impl Clone for ParsingConfiguration {
    fn clone(&self) -> Self {
        Self { metadata: self.metadata.clone(), portability_level: self.portability_level.clone() }
    }
}

impl From<PortabilityLevel> for ParsingConfiguration {
    fn from(value: PortabilityLevel) -> Self {
        Self {
            portability_level: value,
            metadata: ParsingConfigurationMetadata::default()
        }
    }
}