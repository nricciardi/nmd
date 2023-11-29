pub mod parsing_rule;
pub mod parsing_result;
pub mod parsable;

pub use parsing_rule::{ParsingRule, PatternType};
pub use parsing_result::{ParsingResult, ParsingResultBody};

/// Ordered collection of rules
pub struct Codex {
    rules: Vec<ParsingRule>
}

impl Codex {

    pub fn rules(&self) -> &Vec<ParsingRule> {
        &self.rules
    }

    fn new(rules: Vec<ParsingRule>) -> Codex {

        // TODO: check if there are all necessary rules based on theirs type

        Codex {
            rules
        }
    }

    pub fn of_html() -> Codex {
        Codex::new(
            vec![
                ParsingRule::new(PatternType::Bold, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),
                ParsingRule::new(PatternType::Bold, r"__(.*?)__", "<strong>$1</strong>"),
                ParsingRule::new(PatternType::Italic, r"\*(.*?)\*", "<em>$1</em>"),
                ParsingRule::new(PatternType::Italic, r"_(.*?)_", "<em>$1</em>"),
                ParsingRule::new(PatternType::Strikethrough, r"~~(.*?)~~", "<del>$1</del>"),
                ParsingRule::new(PatternType::Underlined, r"\+\+(.*?)\+\+", "<u>$1</u>"),
                ParsingRule::new(PatternType::Link, r"\[([^\]]+)\]\(([^)]+)\)", "<a href=\"$2\">$1</a>"),
                ParsingRule::new(PatternType::Image, r"!\[([^\]]+)\]\(([^)]+)\)", "<img src=\"$2\" alt=\"$1\">"),
                // ParsingRule::new(PatternType::Highlight, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),
                // ParsingRule::new(PatternType::ColoredText, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),
                // ParsingRule::new(PatternType::Emoji, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),
                // ParsingRule::new(PatternType::Superscript, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),
                // ParsingRule::new(PatternType::Subscript, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),
                // ParsingRule::new(PatternType::InlineCode, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),
                ParsingRule::new(PatternType::Comment, r"\*\*(.*?)\*\*", "<!-- $1 -->"),
                // ParsingRule::new(PatternType::Bookmark, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),
                // ParsingRule::new(PatternType::Heading, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),
                ParsingRule::new(PatternType::CodeBlock, r"```(\w+)([\s\S]*?)```", "<pre><code>$2</code></pre>"),
                // ParsingRule::new(PatternType::CommentBlock, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),
                // ParsingRule::new(PatternType::FocusBlock, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),
                // ParsingRule::new(PatternType::MathBlock, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),

            ]
        )
    }

    pub fn of_rawtext() -> Codex {
        Codex::new(
            vec![
                ParsingRule::new(PatternType::Bold, r"\*\*(.*?)\*\*", "$1"),
                ParsingRule::new(PatternType::Bold, r"__(.*?)__", "$1"),
                ParsingRule::new(PatternType::Italic, r"\*(.*?)\*", "$1"),
                ParsingRule::new(PatternType::Italic, r"_(.*?)_", "$1"),
                ParsingRule::new(PatternType::Strikethrough, r"~~(.*?)~~", "$1"),
                ParsingRule::new(PatternType::Underlined, r"\+\+(.*?)\+\+", "$1"),
                ParsingRule::new(PatternType::Link, r"\[([^\]]+)\]\(([^)]+)\)", "$1 ($2)"),
                ParsingRule::new(PatternType::Image, r"!\[([^\]]+)\]\(([^)]+)\)", "[$1: $2]"),
                // ParsingRule::new(PatternType::Highlight, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),
                // ParsingRule::new(PatternType::ColoredText, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),
                // ParsingRule::new(PatternType::Emoji, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),
                // ParsingRule::new(PatternType::Superscript, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),
                // ParsingRule::new(PatternType::Subscript, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),
                // ParsingRule::new(PatternType::InlineCode, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),
                ParsingRule::new(PatternType::Comment, r"\*\*(.*?)\*\*", "// $1"),
                // ParsingRule::new(PatternType::Bookmark, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),
                // ParsingRule::new(PatternType::Heading, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),
                ParsingRule::new(PatternType::CodeBlock, r"```(\w+)([\s\S]*?)```", "\n\n$2\n\n"),
                // ParsingRule::new(PatternType::CommentBlock, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),
                // ParsingRule::new(PatternType::FocusBlock, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),
                // ParsingRule::new(PatternType::MathBlock, r"\*\*(.*?)\*\*", "<strong>$1</strong>"),

            ]
        )
    }
}


#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn multiple_uses() {
        let codex: &Codex = &Codex::of_html();

        let nmd_text = "This is a simple **nmd** text for test";
        let expected_result = "This is a simple <strong>nmd</strong> text for test";
        let mut parsing_result = String::from(nmd_text);

        for rule in codex.rules() {
            let result = rule.parse(parsing_result.as_str()).unwrap();

            parsing_result = result.parsed_content()
        }

        assert_eq!(parsing_result, expected_result);

        let nmd_text = "This is a simple *nmd* text for test";
        let expected_result = "This is a simple <em>nmd</em> text for test";
        let mut parsing_result = String::from(nmd_text);

        for rule in codex.rules() {
            let result = rule.parse(parsing_result.as_str()).unwrap();

            parsing_result = result.parsed_content()
        }

        assert_eq!(parsing_result, expected_result);
    }

}
