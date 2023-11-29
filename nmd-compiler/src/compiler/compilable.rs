use super::parsable::{Parsable, ParsingConfiguration};
use super::location::Locatable;
use anyhow::Result;


pub trait Compilable: Locatable + Parsable {
    fn compile(&self, compilation_configuration: CompilationConfiguration) -> Result<()>;
}

pub struct CompilationConfiguration {
    parsing_configuration: ParsingConfiguration
}