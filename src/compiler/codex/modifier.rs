pub mod paragraph_modifier;
pub mod modifiers_bucket;
pub mod text_modifier;
pub mod chapter_modifier;
pub mod base_modifier;

use std::fmt;

use self::{base_modifier::BaseModifier, modifiers_bucket::ModifiersBucket};


pub const MAX_HEADING_LEVEL: u32 = 6; 

pub type ModifierIdentifier = String;

pub trait Mod: Sync + Send {

    fn identifier(&self) -> &ModifierIdentifier {
        &self.search_pattern()
    }

    fn search_pattern(&self) -> &String;

    fn incompatible_modifiers(&self) -> &ModifiersBucket {
        &ModifiersBucket::None
    }
}

impl fmt::Debug for dyn Mod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.identifier(), self.search_pattern())
    }
}

impl PartialEq for dyn Mod {
    fn eq(&self, other: &Self) -> bool {
        self.search_pattern().eq(other.search_pattern())
    }
}

impl Clone for Box<dyn Mod> {
    fn clone(&self) -> Self {
        Box::new(BaseModifier::new(self.identifier().clone(), self.search_pattern().clone(), self.incompatible_modifiers().clone()))
    }
}

/// NMD modifiers pattern types
#[derive(Debug, PartialEq, Clone)]
pub enum Modifier {

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

    // PARAGRAPH MODIFIERs
    List,
    ListItem,
    Image,
    CodeBlock,
    CommentBlock,
    ExtendedBlockQuote,
    ExtendedBlockQuoteLine,
    FocusBlock,
    MathBlock,
    LineBreakDash,
    LineBreakStar,
    LineBreakPlus,
    CommonParagraph,
    AbridgedEmbeddedParagraphStyleWithId,
    AbridgedEmbeddedParagraphStyle,
    EmbeddedParagraphStyleWithId,
    EmbeddedParagraphStyle,
    ParagraphIdentifier,
    PageBreak,

    Custom
}

impl Modifier {

    pub fn ordered_paragraph_modifiers() -> Vec<Self> {


        //! they must have the compatibility order
        vec![
            Self::AbridgedTodo,
            Self::PageBreak,
            Self::ParagraphIdentifier,
            Self::EmbeddedParagraphStyleWithId,
            Self::EmbeddedParagraphStyle,
            Self::AbridgedEmbeddedParagraphStyleWithId,
            Self::AbridgedEmbeddedParagraphStyle,
            Self::LineBreakDash,
            Self::LineBreakStar,
            Self::LineBreakPlus,
            Self::List,
            Self::Image,
            Self::CodeBlock,
            Self::CommentBlock,
            Self::ExtendedBlockQuote,
            Self::FocusBlock,
            Self::MathBlock,
            Self::CommonParagraph,
        ]
    }

    pub fn heading_modifiers_rev() -> Vec<Self> {
        let mut heading_modifiers: Vec<Self> = Vec::new();

        for i in (1..=MAX_HEADING_LEVEL).rev() {
            heading_modifiers.push(Self::HeadingGeneralExtendedVersion(i));
            heading_modifiers.push(Self::HeadingGeneralCompactVersion(i));
        }

        heading_modifiers
    }

    
}


impl Mod for Modifier {
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
            Self::Image => String::from(r"!\[([^\]]+)\]\(([^)]+)\)"),
            Self::HeadingGeneralExtendedVersion(level) => {

                if level == 0 || level > MAX_HEADING_LEVEL {
                    panic!("{level} is an invalid heading level.")
                }

                format!(r"(?m:^{}\s+(.*))", "#".repeat(level as usize))
            },
            Self::HeadingGeneralCompactVersion(level) => {

                if level == 0 || level > MAX_HEADING_LEVEL {
                    panic!("{level} is an invalid heading level.")
                }

                format!(r"(?m:^#({})\s+(.*))", level)
            },
            Self::InlineCode => String::from(r"`(.*?)`"),
            Self::InlineMath => String::from(r#"\$([^$\n]+)\$"#),

            Self::CommonParagraph => String::from(r#"([\s\S]*?)"#),       // TODO
            Self::CodeBlock => String::from(r"```(\w+)\n+(.*?)\n+```"),
            Self::MathBlock => String::from(r#"\$\$((?s:.+?))\$\$"#),

            Self::ListItem => String::from(r#"(?m:^([\t ]*)(-\[\]|-\[ \]|-\[x\]|-\[X\]|-|->|\||\*|\+|--|\d[\.)]?|[a-zA-Z]{1,8}[\.)]|&[^;]+;) (.*)\n)"#),
            Self::List => format!(r#"((?:{}+)+)"#, Self::ListItem.search_pattern()),
            Self::ExtendedBlockQuoteLine => String::from(r"(?m:^> (.*))"),
            Self::ExtendedBlockQuote => format!(r"({}){}({})?", Self::ExtendedBlockQuoteLine.search_pattern(), String::from(r"\n(?:(?mx:^> .*\n)*)"), Self::ExtendedBlockQuoteLine.search_pattern()),
            Self::LineBreakDash => String::from(r"(?m:^-{3,})"),
            Self::LineBreakStar => String::from(r"(?m:^\*{3,})"),
            Self::LineBreakPlus => String::from(r"(?m:^\+{3,})"),
            Self::FocusBlock => String::from(r":::\s(\w+)\n(?s:(.*?))\n:::"),
            Self::AbridgedEmbeddedParagraphStyle => String::from(r"\[\[(?sx:(.*?))\]\]\{(.*?)(?s:;(.*?)(?:;(.*?))?)?\}"),
            Self::AbridgedEmbeddedParagraphStyleWithId => String::from(r"\[\[(?sx:(.*?))\]\]\n?#([\w-]*)\n?\{(.*?)(?s:;(.*?)(?:;(.*?))?)?\}"),
            Self::ParagraphIdentifier => String::from(r"\[\[(?sx:(.*?))\]\]\n?#([\w-]*)"),
            Self::EmbeddedParagraphStyleWithId => String::from(r"\[\[(?sx:(.*?))\]\]\n?#([\w-]*)\n?\{\{(?xs:((?:.*?:.*?;?)))\}\}"),
            Self::EmbeddedParagraphStyle => String::from(r"\[\[(?sx:(.*?))\]\]\{\{(?xs:((?:.*?:.*?;?)))\}\}"),
            Self::PageBreak => String::from(r"(?m:^#{3,}$)"),
            Self::AbridgedTodo => String::from(r"(?m:^(?i:TODO):\s(?:(.*?))$)"),
            
            _ => String::from(r"RULE TODO")                                               // TODO
        }
    }

    fn incompatible_modifiers(&self) -> &ModifiersBucket {
        &match self {

            Self::Image => ModifiersBucket::All,
            Self::InlineCode => ModifiersBucket::All,
            Self::CodeBlock => ModifiersBucket::All,
            Self::InlineMath => ModifiersBucket::All,
            Self::MathBlock => ModifiersBucket::All,
            Self::Emoji => ModifiersBucket::All,
            _ => ModifiersBucket::None
        }
    }
}