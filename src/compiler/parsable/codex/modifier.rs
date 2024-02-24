use std::{fmt::Display, ops::Add};

use regex::Regex;


pub const MAX_HEADING_LEVEL: u32 = 6; 



#[derive(Debug, PartialEq, Clone)]
pub enum Modifiers {
    All,
    List(Vec<Modifier>),
    None
}

impl Modifiers {
    pub fn contains(&self, searched_modifier: &Modifier) -> bool {
        match self {
            Modifiers::All => true,
            Modifiers::List(modifiers_list) => modifiers_list.contains(searched_modifier),
            Modifiers::None => false,
        }
    }
}

impl Add for Modifiers {
    type Output = Modifiers;

    fn add(self, new_modifiers_excluded: Self) -> Self::Output {
        match new_modifiers_excluded.clone() {
            Modifiers::All => Self::All,
            Modifiers::List(mut modifiers_to_add) => {
                match self {
                    Modifiers::All => return Self::All,
                    Modifiers::List(mut modifiers_already_excluded) => {
                        modifiers_already_excluded.append(&mut modifiers_to_add);

                        return Modifiers::List(modifiers_already_excluded)
                    },
                    Modifiers::None => return new_modifiers_excluded.clone(),
                }
            },
            Modifiers::None => return self
        }
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

    Custom
}

impl Modifier {

    pub fn paragraph_modifiers() -> Vec<Self> {


        //! they must have the compatibility order
        vec![
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

    pub fn heading_level(content: &str) -> Option<u32> {
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

    pub fn is_heading(content: &str) -> bool {
        Self::heading_level(content).is_some()
    }

    pub fn search_pattern(&self) -> String {
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

            Self::CommonParagraph => String::from(r#"(?s:(?m:^(.+?)(?:\n\n|\n$)))"#),
            Self::CodeBlock => String::from(r"```(\w+)\n+(.*?)\n+```"),
            Self::MathBlock => String::from(r#"\$\$((?s:.+?))\$\$"#),

            Self::ListItem => String::from(r#"(?m:^([\t ]*)(-\[\]|-\[ \]|-\[x\]|-\[X\]|-|->|\||\*|\+|--|\d[\.)]?|[a-zA-Z]{1,8}[\.)]|&[^;]+;) (.*))"#),
            Self::List => format!(r"({}){}({})?", Self::ListItem.search_pattern(), String::from(r"\n(?:(?mx:^([\t ]*)(-\[\]|-\[ \]|-\[x\]|-\[X\]|-|->|\||\*|\+|--|\d[\.)]?|[a-zA-Z]{1,8}[\.)]|&[^;]+;) (.*)\n)*)"), Self::ListItem.search_pattern()),
            Self::ExtendedBlockQuoteLine => String::from(r"(?m:^> (.*))"),
            Self::ExtendedBlockQuote => format!("({}){}({})?", Self::ExtendedBlockQuoteLine.search_pattern(), String::from(r"\n(?:(?mx:^> .*\n)*)"), Self::ExtendedBlockQuoteLine.search_pattern()),
            Self::LineBreakDash => String::from(r"(?m:^-{3,})"),
            Self::LineBreakStar => String::from(r"(?m:^\*{3,})"),
            Self::LineBreakPlus => String::from(r"(?m:^\+{3,})"),
            Self::FocusBlock => String::from(r":::\s(\w+)\n(?s:(.*))\n:::"),
            Self::AbridgedEmbeddedParagraphStyle => String::from(r"\[\[(?sx:(.*?))\]\]\{(.*?)(?s:;(.*?)(?:;(.*?))?)?\}"),
            Self::AbridgedEmbeddedParagraphStyleWithId => String::from(r"\[\[(?sx:(.*?))\]\]\n?#([\w-]*)\n?\{(.*?)(?s:;(.*?)(?:;(.*?))?)?\}"),
            Self::ParagraphIdentifier => String::from(r"\[\[(?sx:(.*?))\]\]\n?#([\w-]*)"),
            Self::EmbeddedParagraphStyleWithId => String::from(r"\[\[(?sx:(.*?))\]\]\n?#([\w-]*)\n?\{\{(?xs:((?:.*?:.*?;?)))\}\}"),
            Self::EmbeddedParagraphStyle => String::from(r"\[\[(?sx:(.*?))\]\]\{\{(?xs:((?:.*?:.*?;?)))\}\}"),
            
            _ => String::from(r"RULE TODO")                                               // TODO
        }
    }

    pub fn incompatible_modifiers(&self) -> Modifiers {
        match self {

            Self::Image => Modifiers::All,
            Self::InlineCode => Modifiers::All,
            Self::CodeBlock => Modifiers::All,
            Self::InlineMath => Modifiers::All,
            Self::MathBlock => Modifiers::All,
            Self::Emoji => Modifiers::All,
            _ => Modifiers::None
        }
    }
}