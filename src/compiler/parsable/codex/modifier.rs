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
    Highlight,
    ColoredText,
    Emoji,
    Superscript,
    Subscript,
    InlineCode,
    InlineMath,
    Comment,
    Bookmark,
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
    FocusQuotationBlock,
    FocusBlock,
    MathBlock,
    CommonParagraph,

    Custom
}

impl Modifier {

    pub fn paragraph_modifiers() -> Vec<Self> {


        // they must have the compatibility order
        vec![
            Self::List,
            Self::Image,
            Self::CodeBlock,
            Self::CommentBlock,
            Self::FocusQuotationBlock,
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
            Self::Comment => String::from(r"^//(.*)"),
            Self::Emoji => String::from(r":(.*):"),
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

                format!(r"{}\s+(.*)", "#".repeat(level as usize))
            },
            Self::HeadingGeneralCompactVersion(level) => {

                if level == 0 || level > MAX_HEADING_LEVEL {
                    panic!("{level} is an invalid heading level.")
                }

                format!(r"#({})\s+(.*)", level)
            },
            Self::InlineCode => String::from(r"`(.*?)`"),
            Self::InlineMath => String::from(r#"\$([^$\n]+)\$"#),

            Self::CommonParagraph => String::from(r#"(?s:(?m:^(.+?)(?:\n\n|\n$)))"#),
            Self::CodeBlock => String::from(r"```([a-zA-Z]+)\n+(.*?)\n+```"),
            Self::MathBlock => String::from(r#"\$\$((?s:.+?))\$\$"#),

            Self::ListItem => String::from(r#"(?s)([\t ]*)(-\[\]|-\[ \]|-\[x\]|-\[X\]|-|->|\||\*|\+|--|\d[\.)]?|.{1,8}[\.)]|&[^;]+;) (.*)"#),
            Self::List => format!("({})(?s:.*)({})", Self::ListItem.search_pattern(), Self::ListItem.search_pattern()),
            Self::FocusBlock => String::from(r"^::: (.*)\n(?s:(.*))\n:::"),
            
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