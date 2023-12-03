use crate::compiler::parsable::ParsingConfiguration;

pub struct CompilationConfiguration {
    parsing_configuration: ParsingConfiguration
}

impl Default for CompilationConfiguration {
    fn default() -> Self {
        Self { parsing_configuration: Default::default() }
    }
}

impl CompilationConfiguration {
    pub fn new() -> Self {
        CompilationConfiguration { 
            ..Default::default()
        }
    }
}