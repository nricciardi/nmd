pub mod replacement_rule;


use super::parsable::ParsingConfiguration;
pub use super::parsing_result::{ParsingResult, ParsingError, ParsingResultBody};


/// NMD modifiers pattern types 
pub enum PatternType {
    Bold,
    Italic,
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
    CodeBlock,
    CommentBlock,
    FocusBlock,
    MathBlock,


    Custom
}

pub trait ParsingRule {
    fn parse(&self, content: &str, parsing_configuration: ParsingConfiguration) -> ParsingResult;
}
