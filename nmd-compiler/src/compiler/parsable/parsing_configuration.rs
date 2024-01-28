

#[derive(Clone, Default)]
pub struct ParsingConfigurationMetadata {}

pub struct ParsingConfiguration {
    metadata: ParsingConfigurationMetadata
}

impl ParsingConfiguration {

    pub fn metadata(&self) -> &ParsingConfigurationMetadata {
        &self.metadata
    }

    pub fn new(metadata: ParsingConfigurationMetadata) -> Self {
        Self {
            metadata
        }
    }
}

impl Default for ParsingConfiguration {
    fn default() -> Self {
        ParsingConfiguration {
            metadata: ParsingConfigurationMetadata {  }
        }
    }
}

impl Clone for ParsingConfiguration {
    fn clone(&self) -> Self {
        Self { metadata: self.metadata.clone() }
    }
}