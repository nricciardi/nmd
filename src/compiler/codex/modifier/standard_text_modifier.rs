use super::{base_modifier::BaseModifier, modifiers_bucket::ModifiersBucket, Modifier, ModifierIdentifier};

#[derive(Debug, PartialEq, Clone)]
pub enum StandardTextModifier {

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
    Checkbox,
    CheckboxChecked,
    GreekLetter,
}

impl StandardTextModifier {

    pub fn ordered() -> Vec<Self> {

        //! they must have the compatibility order
        vec![
            Self::GreekLetter,
            Self::Todo,
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

    pub fn identifier(&self) -> ModifierIdentifier {
        match self {
            Self::AbridgedBookmark => String::from("abridged-bookmark"),
            Self::AbridgedBookmarkWithId => String::from("abridged-bookmark-with-id"),
            Self::Bookmark => String::from("bookmark"),
            Self::BookmarkWithId => String::from("bookmark-with-id"),
            Self::Todo => String::from("todo"),
            Self::AbridgedEmbeddedStyle => String::from("abridged-embedded-style"),
            Self::AbridgedEmbeddedStyleWithId => String::from("abridged-embedded-style-with-id"),
            Self::Identifier => String::from("identifier"),
            Self::EmbeddedStyleWithId => String::from("embedded-style-with-id"),
            Self::EmbeddedStyle => String::from("embedded-style"),
            Self::Highlight => String::from("highlight"),
            Self::Comment => String::from("comment"),
            Self::Emoji => String::from("emoji"),
            Self::Checkbox => String::from("checkbox"),
            Self::CheckboxChecked => String::from("checkbox-checked"),
            Self::Superscript => String::from("superscript"),
            Self::Subscript => String::from("subscript"),
            Self::BoldStarVersion => String::from("bold-star-version"),
            Self::BoldUnderscoreVersion => String::from("bold-underscore-version"),
            Self::ItalicStarVersion => String::from("italic-star-version"),
            Self::ItalicUnderscoreVersion => String::from("italic-underscore-version"),
            Self::Strikethrough => String::from("strikethrough"),
            Self::Underlined => String::from("underlined"),
            Self::Link => String::from("link"),
            Self::InlineCode => String::from("inline-code"),
            Self::InlineMath => String::from("inline-math"),
            Self::GreekLetter => String::from("greek-letter"),

            _ => {

                log::warn!("there is NOT a identifier for {:#?}", self);
                String::from("#@§rule-todo#@§")
            }
        }
    }
    
    pub fn modifier_pattern(&self) -> String {
        match *self {
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
            Self::GreekLetter => String::from(r"\\(.*?)\\"),
            
            _ => {
                log::warn!("there is NOT a modifier pattern for {:#?}", self);
                String::from(r"RULE TODO")
            }                                               // TODO
        }
    }

    pub fn incompatible_modifiers(&self) -> ModifiersBucket {
        match self {

            Self::InlineCode => ModifiersBucket::All,
            Self::InlineMath => ModifiersBucket::All,
            Self::Emoji => ModifiersBucket::All,
            Self::GreekLetter => ModifiersBucket::All,
            _ => ModifiersBucket::None
        }
    }
}


impl Into<BaseModifier> for StandardTextModifier {
    fn into(self) -> BaseModifier {
        BaseModifier::new(self.identifier(), self.modifier_pattern(), self.incompatible_modifiers())
    }
}