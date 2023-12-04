use crate::compiler::supported_format::SupportedFormat;

pub use super::codex::Codex;


pub enum PortabilityLevel {
    AllInOne,
    // TODO
}


pub struct Metadata {}

pub struct ParsingConfiguration {
    codex: Codex,
    metadata: Metadata,
    portability_level: PortabilityLevel
}

impl ParsingConfiguration {
    pub fn codex(&self) -> &Codex {
        &self.codex
    }

    pub fn new(format: SupportedFormat, metadata: Metadata, portability_level: PortabilityLevel) -> Self {
        Self {
            codex: Codex::from(format),
            metadata,
            portability_level
        }
    }
}

impl Default for ParsingConfiguration {
    fn default() -> Self {
        ParsingConfiguration {
            codex: Codex::of_html(),
            metadata: Metadata {  },
            portability_level: PortabilityLevel::AllInOne
        }
    }
}

impl From<SupportedFormat> for ParsingConfiguration {
    fn from(format: SupportedFormat) -> Self {
        Self {
            codex: Codex::from(format),
            ..Default::default()
        }
    }
}