pub mod parsing_rule;

pub use parsing_rule::{ParsingRule, ParsingResult, ParsingResultBody}; 

/// Ordered collection of rules
pub struct Codex {
    rules: Vec<ParsingRule>
}