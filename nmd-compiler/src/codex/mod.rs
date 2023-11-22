pub mod parsing_rule;
pub mod parsing_result;

pub use parsing_rule::ParsingRule;
pub use parsing_result::{ParsingResult, ParsingResultBody};

/// Ordered collection of rules
pub struct Codex {
    rules: Vec<ParsingRule>
}


/// return a Codex
pub trait CodexFactory {
    fn create() -> Codex;
}

pub struct HtmlCodexFactory {

}

impl CodexFactory for HtmlCodexFactory {
    fn create() -> Codex {
        Codex {
            rules: vec![

            ]
        }
    }
}