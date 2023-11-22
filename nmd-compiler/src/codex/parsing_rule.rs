pub use super::parsing_result::ParsingResult;



/// Rule to parse a NMD text based on a specific pattern matching rule
pub struct ParsingRule {
    matching_rule: String
}

impl ParsingRule {
    pub fn new(matching_rule: String) {
        ParsingRule {
            matching_rule
        }
    }

    pub fn parse(content: &str) -> ParsingResult {
        todo!(format!("will be parsed {content}"))
    }
}