use super::{Codex, ParsingResult};


pub trait Parsable {
    fn parse(&self, parsing_configuration: ParsingConfiguration) -> ParsingResult;
}


pub struct ParsingConfiguration {
    codex: Codex,
    all_in_one: bool
}

impl ParsingConfiguration {
    pub fn codex(&self) -> &Codex {
        &self.codex
    }

    pub fn all_in_one(&self) -> &bool {
        &self.all_in_one
    }
}