pub mod replacement_rule;
pub mod parsing_result;


use std::sync::{Arc, RwLock};

use crate::compiler::parsable::ParsingConfiguration;

use self::parsing_result::{ParsingOutcome, ParsingError};


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
    HeadingGeneralCompactVersion(i32),
    HeadingGeneralExtendedVersion(i32),
    Heading1ExtendedVersion,
    Heading2ExtendedVersion,
    Heading3ExtendedVersion,
    Heading4ExtendedVersion,
    Heading5ExtendedVersion,
    Heading6ExtendedVersion,
    CodeBlock,
    CommentBlock,
    FocusBlock,
    MathBlock,


    Custom
}

impl Modifier {
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
            Self::Heading6ExtendedVersion | Self::HeadingGeneralExtendedVersion(6) => String::from(r"######\s+(.*)"),
            Self::Heading5ExtendedVersion | Self::HeadingGeneralExtendedVersion(5) => String::from(r"#####\s+(.*)"),
            Self::Heading4ExtendedVersion | Self::HeadingGeneralExtendedVersion(4) => String::from(r"####\s+(.*)"),
            Self::Heading3ExtendedVersion | Self::HeadingGeneralExtendedVersion(3) => String::from(r"###\s+(.*)"),
            Self::Heading2ExtendedVersion | Self::HeadingGeneralExtendedVersion(2) => String::from(r"##\s+(.*)"),
            Self::Heading1ExtendedVersion | Self::HeadingGeneralExtendedVersion(1) => String::from(r"#\s+(.*)"),
            Self::HeadingGeneralExtendedVersion(level) => format!(r"{}\s+(.*)", "#".repeat(level as usize)),
            Self::HeadingGeneralCompactVersion(level) => format!(r"#({})\s+(.*)", level),
            _ => String::from(r"RULE TODO")                                               // TODO
        }
    }
}


pub trait ParsingRule: Send + Sync {
    fn parse(&self, content: &str, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError>;
}
