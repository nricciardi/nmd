pub mod parsing_rule;


use std::sync::Arc;

pub use parsing_rule::{ParsingRule, Modifier};
use crate::compiler::supported_format::SupportedFormat;
use self::parsing_rule::parsing_result::{ParsingError, ParsingOutcome};
use self::parsing_rule::replacement_rule::ReplacementRule;
use super::ParsingConfiguration;


/// Ordered collection of rules
pub struct Codex {
    rules: Vec<Box<dyn ParsingRule>>
}

impl Codex {

    pub fn rules(&self) -> &Vec<Box<dyn ParsingRule>> {
        &self.rules
    }

    pub fn parse(&self, content: &str, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError> {

        let mut outcome = ParsingOutcome::new(String::from(content));

        for rule in self.rules() {
            outcome = rule.parse(&outcome.parsed_content(), Arc::clone(&parsing_configuration))?;
        }

        Ok(outcome)
    }

    fn new(rules: Vec<Box<dyn ParsingRule>>) -> Codex {

        // TODO: check if there are all necessary rules based on theirs type

        Codex {
            rules
        }
    }

    pub fn of_html() -> Self {

        let mut rules: Vec<Box<dyn ParsingRule>> = Vec::new();

        for i in (1..=32).rev() {
            rules.push(Box::new(ReplacementRule::new(Modifier::HeadingGeneralExtendedVersion(i), format!(r#"<h{} class="h{}">$1</h{}>"#, i, i, i))));
            rules.push(Box::new(ReplacementRule::new(Modifier::HeadingGeneralCompactVersion(i), String::from(r#"<h$1 class="h$1">$2</h$1>"#))));
        }

        rules.append(&mut vec![
            Box::new(ReplacementRule::new(Modifier::BoldStarVersion, String::from(r#"<strong>$1</strong>"#))),
            Box::new(ReplacementRule::new(Modifier::BoldUnderscoreVersion, String::from(r#"<strong>$1</strong>"#))),
            Box::new(ReplacementRule::new(Modifier::ItalicStarVersion, String::from(r#"<em>$1</em>"#))),
            Box::new(ReplacementRule::new(Modifier::ItalicUnderscoreVersion, String::from(r#"<em>$1</em>"#))),
            Box::new(ReplacementRule::new(Modifier::Strikethrough, String::from(r#"<del>$1</del>"#))),
            Box::new(ReplacementRule::new(Modifier::Underlined, String::from(r#"<u>$1</u>"#))),
            Box::new(ReplacementRule::new(Modifier::Link, String::from(r#"<a href=\"$2\">$1</a>"#))),
            Box::new(ReplacementRule::new(Modifier::Image, String::from(r#"<img src=\"$2\" alt=\"$1\">"#))),
            // Box::new(ReplacementRule::new(PatternType::Highlight, r"\*\*(.*?)\*\*", "<strong>$1</strong>")),
            // Box::new(ReplacementRule::new(PatternType::ColoredText, r"\*\*(.*?)\*\*", "<strong>$1</strong>")),
            // Box::new(ReplacementRule::new(PatternType::Emoji, r"\*\*(.*?)\*\*", "<strong>$1</strong>")),
            // Box::new(ReplacementRule::new(PatternType::Superscript, r"\*\*(.*?)\*\*", "<strong>$1</strong>")),
            // Box::new(ReplacementRule::new(PatternType::Subscript, r"\*\*(.*?)\*\*", "<strong>$1</strong>")),
            // Box::new(ReplacementRule::new(PatternType::InlineCode, r"\*\*(.*?)\*\*", "<strong>$1</strong>")),
            Box::new(ReplacementRule::new(Modifier::Comment, String::from(r#"<!-- $1 -->"#))),
            // Box::new(ReplacementRule::new(PatternType::Bookmark, r"\*\*(.*?)\*\*", "<strong>$1</strong>")),
            // Box::new(ReplacementRule::new(PatternType::Heading, r"\*\*(.*?)\*\*", "<strong>$1</strong>")),
            // Box::new(ReplacementRule::new(PatternType::CodeBlock, r"```(\w+)([\s\S]*?)```", "<pre><code>$2</code></pre>")),
            // Box::new(ReplacementRule::new(PatternType::CommentBlock, r"\*\*(.*?)\*\*", "<strong>$1</strong>")),
            // Box::new(ReplacementRule::new(PatternType::FocusBlock, r"\*\*(.*?)\*\*", "<strong>$1</strong>")),
            // Box::new(ReplacementRule::new(PatternType::MathBlock, r"\*\*(.*?)\*\*", "<strong>$1</strong>")),
        ]);

        Codex::new(rules)
    }
}

impl From<SupportedFormat> for Codex {
    fn from(format: SupportedFormat) -> Self {
        match format {
            SupportedFormat::Html => Self::of_html()
        }
    }
}


#[cfg(test)]
mod test {

    use std::sync::Arc;

    use crate::compiler::parsable::ParsingConfiguration;

    use super::*;

    #[test]
    fn multiple_uses() {
        let codex: &Codex = &Codex::of_html();

        let nmd_text = "This is a simple **nmd** text for test";
        let expected_result = "This is a simple <strong>nmd</strong> text for test";
        let mut parsing_result = String::from(nmd_text);
        let parsing_configuration = Arc::new(ParsingConfiguration::default());

        for rule in codex.rules() {
            let result = rule.parse(parsing_result.as_str(), Arc::clone(&parsing_configuration)).unwrap();

            parsing_result = result.parsed_content()
        }

        assert_eq!(parsing_result, expected_result);

        let nmd_text = "This is a simple *nmd* text for test";
        let expected_result = "This is a simple <em>nmd</em> text for test";
        let mut parsing_result = String::from(nmd_text);

        for rule in codex.rules() {
            let result = rule.parse(parsing_result.as_str(), Arc::clone(&parsing_configuration)).unwrap();

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
<h1 class="h1">title 1</h1>
<h2 class="h2">title 2</h2>
<h6 class="h6">title 6</h6>
"#.trim();

        let mut parsing_result = String::from(nmd_text);
        let parsing_configuration = Arc::new(ParsingConfiguration::default());

        for rule in codex.rules() {
            let result = rule.parse(parsing_result.as_str(), Arc::clone(&parsing_configuration)).unwrap();

            parsing_result = result.parsed_content()
        }

        assert_eq!(parsing_result, expected_result);
    }
}