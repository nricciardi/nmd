
#[derive(Clone)]
pub enum PortabilityLevel {
    AllInOne,
    // TODO
}

#[derive(Clone)]
pub struct Metadata {}

pub struct ParsingConfiguration {
    metadata: Metadata,
    portability_level: PortabilityLevel
}

impl ParsingConfiguration {

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn portability_level(&self) -> &PortabilityLevel {
        &self.portability_level
    }

    pub fn new(metadata: Metadata, portability_level: PortabilityLevel) -> Self {
        Self {
            metadata,
            portability_level
        }
    }
}

impl Default for ParsingConfiguration {
    fn default() -> Self {
        ParsingConfiguration {
            metadata: Metadata {  },
            portability_level: PortabilityLevel::AllInOne
        }
    }
}

impl Clone for ParsingConfiguration {
    fn clone(&self) -> Self {
        Self { metadata: self.metadata.clone(), portability_level: self.portability_level.clone() }
    }
}