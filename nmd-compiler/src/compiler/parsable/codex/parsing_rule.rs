pub mod replacement_rule;
pub mod parsing_result;


use std::sync::Arc;

use regex::Regex;

use crate::compiler::parsable::ParsingConfiguration;

use self::parsing_result::{ParsingOutcome, ParsingError};

pub const MAX_HEADING_LEVEL: u32 = 6; 

/// NMD modifiers pattern types
#[derive(Debug)]
pub enum Modifier { 
    BoldStarVersion,
    BoldUnderscoreVersion,
    ItalicStarVersion,
    ItalicUnderscoreVersion,
    Strikethrough,
    Underlined,
    Link,
    Image,
    Highlight,
    ColoredText,
    Emoji,
    Superscript,
    Subscript,
    InlineCode,
    Comment,
    Bookmark,
    HeadingGeneralCompactVersion(u32),
    HeadingGeneralExtendedVersion(u32),
    /* DEPRECATED: Heading1ExtendedVersion,
    Heading2ExtendedVersion,
    Heading3ExtendedVersion,
    Heading4ExtendedVersion,
    Heading5ExtendedVersion,
    Heading6ExtendedVersion, */
    CodeBlock,
    CommentBlock,
    FocusBlock,
    MathBlock,


    Custom
}

impl Modifier {

    pub fn heading_modifiers_rev() -> Vec<Self> {
        let mut heading_modifiers: Vec<Self> = Vec::new();

        for i in (1..=MAX_HEADING_LEVEL).rev() {
            heading_modifiers.push(Self::HeadingGeneralExtendedVersion(i));
            heading_modifiers.push(Self::HeadingGeneralCompactVersion(i));
        }

        heading_modifiers
    }

    pub fn is_heading(content: &str) -> Option<u32> {
        let heading_modifiers = Self::heading_modifiers_rev();

        for heading_modifier in heading_modifiers {
            let regex = Regex::new(&heading_modifier.search_pattern()).unwrap();

            if regex.is_match(content) {
                match heading_modifier {
                    Self::HeadingGeneralExtendedVersion(level) => return Option::Some(level),
                    Self::HeadingGeneralCompactVersion(level) => return Option::Some(level),
                    _ => panic!("unexpected modifier: {:?}", heading_modifier)
                }
            }
        }

        Option::None
    }

    pub fn search_pattern(&self) -> String {
        match *self {
            Self::BoldStarVersion => String::from(r"\*\*(.*?)\*\*"),
            Self::BoldUnderscoreVersion => String::from(r"__(.*?)__"),
            Self::ItalicStarVersion => String::from(r"\*(.*?)\*"),
            Self::ItalicUnderscoreVersion => String::from(r"_(.*?)_"),
            Self::Strikethrough => String::from(r"~~(.*?)~~"),
            Self::Underlined => String::from(r"\+\+(.*?)\+\+"),
            Self::Link => String::from(r"\[([^\]]+)\]\(([^)]+)\)"),
            Self::Image => String::from(r"!\[([^\]]+)\]\(([^)]+)\)"),
            Self::HeadingGeneralExtendedVersion(level) => {

                if level == 0 || level > MAX_HEADING_LEVEL {
                    panic!("{level} is an invalid heading level.")
                }

                format!(r"{}\s+(.*)", "#".repeat(level as usize))
            },
            Self::HeadingGeneralCompactVersion(level) => {

                if level == 0 || level > MAX_HEADING_LEVEL {
                    panic!("{level} is an invalid heading level.")
                }

                format!(r"#({})\s+(.*)", level)
            },
            _ => String::from(r"RULE TODO")                                               // TODO
        }
    }
}


pub trait ParsingRule: Send + Sync {

    fn modifier(&self) -> &Modifier;

    fn parse(&self, content: &str, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError>;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_heading() {
        let content = "#6 title 1";

        assert!(Modifier::is_heading(content).is_some());

        let content = "### title 3";

        assert!(Modifier::is_heading(content).is_some());

        let content = "text";

        assert!(Modifier::is_heading(content).is_none())
    }
}