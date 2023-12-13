pub mod replacement_rule;
pub mod parsing_result;


use std::sync::Arc;

use crate::compiler::parsable::ParsingConfiguration;

use self::parsing_result::{ParsingOutcome, ParsingError};

pub const MAX_HEADING_LEVEL: u32 = 32; 

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

    pub fn heading_modifiers() -> Vec<Self> {
        let mut heading_modifiers: Vec<Self> = Vec::new();

        for i in (1..=MAX_HEADING_LEVEL).rev() {
            heading_modifiers.push(Modifier::HeadingGeneralExtendedVersion(i));
            heading_modifiers.push(Modifier::HeadingGeneralCompactVersion(i));
        }

        heading_modifiers
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
