use super::{base_modifier::BaseModifier, constants::NEW_LINE, modifiers_bucket::ModifiersBucket, Modifier, ModifierIdentifier};

#[derive(Debug, PartialEq, Clone)]
pub enum StandardTextModifier {
    
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
    Escape,
    Reference,
    Cite,
}

impl StandardTextModifier {

    pub fn ordered() -> Vec<Self> {

        //! they must have the compatibility order
        vec![
            Self::InlineMath,
            Self::InlineCode,
            Self::Comment,
            Self::Escape,
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
            Self::BoldStarVersion,
            Self::BoldUnderscoreVersion,
            Self::ItalicStarVersion,
            Self::ItalicUnderscoreVersion,
            Self::Strikethrough,
            Self::Underlined,
            Self::Superscript,
            Self::Subscript,
            Self::Link,
            Self::Checkbox,
            Self::CheckboxChecked,
            Self::Emoji,
            Self::Reference,
            Self::Cite,
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
            Self::Escape => String::from("escape"),
            Self::Reference => String::from("reference"),
            Self::Cite => String::from("cite"),

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
            Self::AbridgedEmbeddedStyleWithId => format!(r"\[([^\]]*?)\]{}?#([\w-]*){}?\{{(.*?)(?s:;(.*?)(?:;(.*?))?)?\}}", NEW_LINE, NEW_LINE),
            Self::Identifier => format!(r"\[(.*?)\]{}?#([\w-]*)", NEW_LINE),
            Self::EmbeddedStyleWithId => format!(r"\[([^\]]*?)\]{}?#([\w-]*){}?\{{\{{(?xs:((?:.*?:.*?;?)))\}}\}}", NEW_LINE, NEW_LINE),
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
            Self::InlineMath => format!(r#"\$([^${}]+)\$"#, NEW_LINE),
            Self::GreekLetter => String::from(r"%(.*?)%"),        // if it changes, fix greek letters rules
            Self::Escape => String::from(r"\\([\*\+\\~%\^\$@=\[\]!<>\{\}\(\)#-_\|\?&]+)"),
            Self::Reference => String::from(r"&([\w-]+)&"),
            Self::Cite => String::from(r"\^\[([\w_]+)\]"),
            
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
            Self::Escape => ModifiersBucket::All,
            Self::Reference => ModifiersBucket::All,
            Self::Cite => ModifiersBucket::All,
            _ => ModifiersBucket::None
        }
    }
}


impl Into<BaseModifier> for StandardTextModifier {
    fn into(self) -> BaseModifier {
        BaseModifier::new(self.identifier(), self.modifier_pattern(), self.incompatible_modifiers())
    }
}