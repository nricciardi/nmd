pub mod parsing_rule;

pub use parsing_rule::{ParsingRule, PatternType};
pub use super::parsing_result::{ParsingResult, ParsingResultBody};

use self::parsing_rule::replacement_rule::ReplacementRule;

/// Ordered collection of rules
pub struct Codex {
    rules: Vec<Box<dyn ParsingRule>>
}

impl Codex {

    pub fn rules(&self) -> &Vec<Box<dyn ParsingRule>> {
        &self.rules
    }

    fn new(rules: Vec<Box<dyn ParsingRule>>) -> Codex {

        // TODO: check if there are all necessary rules based on theirs type

        Codex {
            rules
        }
    }

    pub fn of_html() -> Codex {
        Codex::new(
            vec![
                Box::new(ReplacementRule::new(PatternType::HeadingH6, "<h6>$1</h6>")),
                Box::new(ReplacementRule::new(PatternType::HeadingH5, "<h5>$1</h5>")),
                Box::new(ReplacementRule::new(PatternType::HeadingH4, "<h4>$1</h4>")),
                Box::new(ReplacementRule::new(PatternType::HeadingH3, "<h3>$1</h3>")),
                Box::new(ReplacementRule::new(PatternType::HeadingH2, "<h2>$1</h2>")),
                Box::new(ReplacementRule::new(PatternType::HeadingH1, "<h1>$1</h1>")),
                Box::new(ReplacementRule::new(PatternType::Heading, "<h$1>$2</h$1>")),
                Box::new(ReplacementRule::new(PatternType::BoldV1, "<strong>$1</strong>")),
                Box::new(ReplacementRule::new(PatternType::BoldV2, "<strong>$1</strong>")),
                Box::new(ReplacementRule::new(PatternType::ItalicV1, "<em>$1</em>")),
                Box::new(ReplacementRule::new(PatternType::ItalicV2, "<em>$1</em>")),
                Box::new(ReplacementRule::new(PatternType::Strikethrough, "<del>$1</del>")),
                Box::new(ReplacementRule::new(PatternType::Underlined, "<u>$1</u>")),
                Box::new(ReplacementRule::new(PatternType::Link, "<a href=\"$2\">$1</a>")),
                Box::new(ReplacementRule::new(PatternType::Image, "<img src=\"$2\" alt=\"$1\">")),
                // Box::new(ReplacementRule::new(PatternType::Highlight, r"\*\*(.*?)\*\*", "<strong>$1</strong>")),
                // Box::new(ReplacementRule::new(PatternType::ColoredText, r"\*\*(.*?)\*\*", "<strong>$1</strong>")),
                // Box::new(ReplacementRule::new(PatternType::Emoji, r"\*\*(.*?)\*\*", "<strong>$1</strong>")),
                // Box::new(ReplacementRule::new(PatternType::Superscript, r"\*\*(.*?)\*\*", "<strong>$1</strong>")),
                // Box::new(ReplacementRule::new(PatternType::Subscript, r"\*\*(.*?)\*\*", "<strong>$1</strong>")),
                // Box::new(ReplacementRule::new(PatternType::InlineCode, r"\*\*(.*?)\*\*", "<strong>$1</strong>")),
                Box::new(ReplacementRule::new(PatternType::Comment,"<!-- $1 -->")),
                // Box::new(ReplacementRule::new(PatternType::Bookmark, r"\*\*(.*?)\*\*", "<strong>$1</strong>")),
                // Box::new(ReplacementRule::new(PatternType::Heading, r"\*\*(.*?)\*\*", "<strong>$1</strong>")),
                // Box::new(ReplacementRule::new(PatternType::CodeBlock, r"```(\w+)([\s\S]*?)```", "<pre><code>$2</code></pre>")),
                // Box::new(ReplacementRule::new(PatternType::CommentBlock, r"\*\*(.*?)\*\*", "<strong>$1</strong>")),
                // Box::new(ReplacementRule::new(PatternType::FocusBlock, r"\*\*(.*?)\*\*", "<strong>$1</strong>")),
                // Box::new(ReplacementRule::new(PatternType::MathBlock, r"\*\*(.*?)\*\*", "<strong>$1</strong>")),

            ]
        )
    }
}


#[cfg(test)]
mod test {

    use crate::compiler::parsable::ParsingConfiguration;

    use super::*;

    #[test]
    fn multiple_uses() {
        let codex: &Codex = &Codex::of_html();

        let nmd_text = "This is a simple **nmd** text for test";
        let expected_result = "This is a simple <strong>nmd</strong> text for test";
        let mut parsing_result = String::from(nmd_text);
        let parsing_configuration = ParsingConfiguration::default();

        for rule in codex.rules() {
            let result = rule.parse(parsing_result.as_str(), &parsing_configuration).unwrap();

            parsing_result = result.parsed_content()
        }

        assert_eq!(parsing_result, expected_result);

        let nmd_text = "This is a simple *nmd* text for test";
        let expected_result = "This is a simple <em>nmd</em> text for test";
        let mut parsing_result = String::from(nmd_text);

        for rule in codex.rules() {
            let result = rule.parse(parsing_result.as_str(), &parsing_configuration).unwrap();

            parsing_result = result.parsed_content()
        }

        assert_eq!(parsing_result, expected_result);
    }

    #[test]
    fn headings () {
        let codex: &Codex = &Codex::of_html();

        let nmd_text = 
r#"
#1 title 1
## title 2
###### title 6
"#.trim();
        let expected_result = 
r#"
<h1>title 1</h1>
<h2>title 2</h2>
<h6>title 6</h6>
"#.trim();

        let mut parsing_result = String::from(nmd_text);
        let parsing_configuration = ParsingConfiguration::default();

        for rule in codex.rules() {
            let result = rule.parse(parsing_result.as_str(), &parsing_configuration).unwrap();

            parsing_result = result.parsed_content()
        }

        assert_eq!(parsing_result, expected_result);
    }
}