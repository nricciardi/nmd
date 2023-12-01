pub use super::codex::Codex;

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

impl Default for ParsingConfiguration {
    fn default() -> Self {
        ParsingConfiguration { codex: Codex::of_html(), all_in_one: true }
    }
}