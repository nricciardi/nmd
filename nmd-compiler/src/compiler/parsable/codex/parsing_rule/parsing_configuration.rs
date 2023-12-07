use crate::compiler::supported_format::SupportedFormat;

pub use super::super::Codex;


pub enum PortabilityLevel {
    AllInOne,
    // TODO
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParallelizationLevel {     // TODO: maybe in ParsingConfiguration
    None,
    Low,
    Medium,
    High,
    Extreme
}

pub struct Metadata {}

pub struct ParsingConfiguration {
    codex: Codex,
    metadata: Metadata,
    portability_level: PortabilityLevel,
    parallelization_level: ParallelizationLevel
}

impl ParsingConfiguration {
    pub fn codex(&self) -> &Codex {
        &self.codex
    }

    pub fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    pub fn portability_level(&self) -> &PortabilityLevel {
        &self.portability_level
    }

    pub fn parallelization_level(&self) -> &ParallelizationLevel {
        &self.parallelization_level
    }

    pub fn new(format: SupportedFormat, metadata: Metadata, portability_level: PortabilityLevel, parallelization_level: ParallelizationLevel) -> Self {
        Self {
            codex: Codex::from(format),
            metadata,
            portability_level,
            parallelization_level
        }
    }
}

impl Default for ParsingConfiguration {
    fn default() -> Self {
        ParsingConfiguration {
            codex: Codex::of_html(),
            metadata: Metadata {  },
            portability_level: PortabilityLevel::AllInOne,
            parallelization_level: ParallelizationLevel::None
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