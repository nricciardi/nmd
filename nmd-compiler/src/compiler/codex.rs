pub mod parsing_rule;
pub mod parsing_result;

pub use parsing_rule::ParsingRule;
pub use parsing_result::{ParsingResult, ParsingResultBody};

/// Ordered collection of rules
pub struct Codex {
    rules: Vec<ParsingRule>
}

impl Codex {
    fn of_html() -> Codex {
        Codex {
            rules: vec![

            ]
        }
    }
}

