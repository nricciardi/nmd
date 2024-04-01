use super::{modifiers_bucket::ModifiersBucket, Mod};

#[derive(Debug, PartialEq, Clone)]
pub enum TextModifier {

    // CONTENT MODIFIERs
    BoldStarVersion,
    BoldUnderscoreVersion,
    ItalicStarVersion,
    ItalicUnderscoreVersion,
    Strikethrough,
    Underlined,
    Link,
    AbridgedEmbeddedStyleWithId,
    AbridgedEmbeddedStyle,
    EmbeddedStyleWithId,
    EmbeddedStyle,
    Identifier,
    Highlight,
    ColoredText,
    Emoji,
    Superscript,
    Subscript,
    InlineCode,
    InlineMath,
    Comment,
    AbridgedBookmark,
    Bookmark,
    AbridgedBookmarkWithId,
    BookmarkWithId,
    Todo,
    AbridgedTodo,
    Checkbox,
    CheckboxChecked,
    HeadingGeneralCompactVersion(u32),
    HeadingGeneralExtendedVersion(u32),
}

impl TextModifier {

    pub fn ordered() -> Vec<Self> {

        //! they must have the compatibility order
        vec![
            Self::Todo,
            Self::AbridgedTodo,
            Self::BookmarkWithId,
            Self::Bookmark,
            Self::AbridgedBookmarkWithId,
            Self::AbridgedBookmark,
            Self::EmbeddedStyleWithId,
            Self::EmbeddedStyle,
            Self::AbridgedEmbeddedStyleWithId,
            Self::AbridgedEmbeddedStyle,
            Self::Identifier,
            Self::Highlight,
            Self::InlineMath,
            Self::InlineCode,
            Self::BoldStarVersion,
            Self::BoldUnderscoreVersion,
            Self::ItalicStarVersion,
            Self::ItalicUnderscoreVersion,
            Self::Strikethrough,
            Self::Underlined,
            Self::Superscript,
            Self::Subscript,
            Self::Link,
            Self::Comment,
            Self::Checkbox,
            Self::CheckboxChecked,
            Self::Emoji,
        ]
    }
}


impl Mod for TextModifier {
    
    fn search_pattern(&self) -> &String {
        &match *self {
            Self::AbridgedBookmark => String::from(r"@\[([^\]]*?)\]"),
            Self::AbridgedBookmarkWithId => String::from(r"@\[([^\]]*?)\]#([\w-]*)"),
            Self::Bookmark => String::from(r"@\[([^\]]*?)\]\((?s:(.*?))\)"),
            Self::BookmarkWithId => String::from(r"@\[([^\]]*?)\]#([\w-]*)\((?s:(.*?))\)"),
            Self::Todo => String::from(r"@\[(?i:TODO)\]\((?s:(.*?))\)"),
            Self::AbridgedEmbeddedStyle => String::from(r"\[([^\]]*?)\]\{(.*?)(?s:;(.*?)(?:;(.*?))?)?\}"),
            Self::AbridgedEmbeddedStyleWithId => String::from(r"\[([^\]]*?)\]\n?#([\w-]*)\n?\{(.*?)(?s:;(.*?)(?:;(.*?))?)?\}"),
            Self::Identifier => String::from(r"\[(.*?)\]\n?#([\w-]*)"),
            Self::EmbeddedStyleWithId => String::from(r"\[([^\]]*?)\]\n?#([\w-]*)\n?\{\{(?xs:((?:.*?:.*?;?)))\}\}"),
            Self::EmbeddedStyle => String::from(r"\[([^\]]*?)\]\{\{(?xs:((?:.*?:.*?;?)))\}\}"),
            Self::Highlight => String::from(r"==(.*)=="),
            Self::Comment => String::from(r"^//(.*)"),
            Self::Emoji => String::from(r":(\w*):"),
            Self::Checkbox => String::from(r"(\[\]|\[ \])"),
            Self::CheckboxChecked => String::from(r"(\[x\]|\[X\])"),
            Self::Superscript => String::from(r"\^(.*)\^"),
            Self::Subscript => String::from(r"~(.*)~"),
            Self::BoldStarVersion => String::from(r"\*\*(.*?)\*\*"),
            Self::BoldUnderscoreVersion => String::from(r"__(.*?)__"),
            Self::ItalicStarVersion => String::from(r"\*(.*?)\*"),
            Self::ItalicUnderscoreVersion => String::from(r"_(.*?)_"),
            Self::Strikethrough => String::from(r"~~(.*?)~~"),
            Self::Underlined => String::from(r"\+\+(.*?)\+\+"),
            Self::Link => String::from(r"\[([^\]]+)\]\(([^)]+)\)"),
            Self::InlineCode => String::from(r"`(.*?)`"),
            Self::InlineMath => String::from(r#"\$([^$\n]+)\$"#),
            
            _ => String::from(r"RULE TODO")                                               // TODO
        }
    }

    fn incompatible_modifiers(&self) -> &ModifiersBucket {
        &match self {

            Self::InlineCode => ModifiersBucket::All,
            Self::InlineMath => ModifiersBucket::All,
            Self::Emoji => ModifiersBucket::All,
            _ => ModifiersBucket::None
        }
    }
}