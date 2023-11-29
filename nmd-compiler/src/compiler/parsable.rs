use super::codex::Codex;


pub trait Parsable {
    fn parse(&self, parsing_configuration: ParsingConfiguration);      // TODO: return type
}


pub struct ParsingConfiguration {
    all_in_one: bool,
    codex: Codex
}