pub mod replacement_rule;
pub mod parsing_result;


use std::sync::{Arc, RwLock};

use crate::compiler::parsable::ParsingConfiguration;

use self::parsing_result::{ParsingOutcome, ParsingError};


/// NMD modifiers pattern types
#[derive(Debug)]
pub enum PatternType { 
    BoldV1,
    BoldV2,
    ItalicV1,
    ItalicV2,
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
    Heading,
    HeadingH1,
    HeadingH2,
    HeadingH3,
    HeadingH4,
    HeadingH5,
    HeadingH6,
    CodeBlock,
    CommentBlock,
    FocusBlock,
    MathBlock,


    Custom
}

impl PatternType {
    pub fn search_pattern(&self) -> &'static str {
        match *self {
            Self::BoldV1 => r"\*\*(.*?)\*\*",
            Self::BoldV2 => r"__(.*?)__",
            Self::ItalicV1 => r"\*(.*?)\*",
            Self::ItalicV2 => r"_(.*?)_",
            Self::Strikethrough => r"~~(.*?)~~",
            Self::Underlined => r"\+\+(.*?)\+\+",
            Self::Link => r"\[([^\]]+)\]\(([^)]+)\)",
            Self::Image => r"!\[([^\]]+)\]\(([^)]+)\)",
            Self::HeadingH6 => r"######\s+(.*)",
            Self::HeadingH5 => r"#####\s+(.*)",
            Self::HeadingH4 => r"####\s+(.*)",
            Self::HeadingH3 => r"###\s+(.*)",
            Self::HeadingH2 => r"##\s+(.*)",
            Self::HeadingH1 => r"#\s+(.*)",
            Self::Heading => r"#(\d+)\s+(.*)",
            _ => r"RULE TODO"                                               // TODO
        }
    }
}


pub trait ParsingRule: Send + Sync {
    fn parse(&self, content: &str, parsing_configuration: Arc<ParsingConfiguration>) -> Result<ParsingOutcome, ParsingError>;
}
