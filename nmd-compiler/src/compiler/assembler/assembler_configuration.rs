use crate::compiler::portability_level::PortabilityLevel;

#[derive(Debug)]
pub struct AssemblerConfiguration {
    portability_level: PortabilityLevel
}

impl From<PortabilityLevel> for AssemblerConfiguration {
    fn from(value: PortabilityLevel) -> Self {
        Self { portability_level: value }
    }
}