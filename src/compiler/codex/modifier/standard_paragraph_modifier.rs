use super::{base_modifier::BaseModifier, constants::NEW_LINE_PATTERN, modifiers_bucket::ModifiersBucket, Modifier, ModifierIdentifier, ModifierPattern};


pub const PARAGRAPH_SEPARATOR_START: &str = r"(?m:^[ \t]*\r*\n)+";
pub const PARAGRAPH_SEPARATOR_END: &str = r"(?m:[ \t]*\r*\n){2}";


#[derive(Debug, PartialEq, Clone)]
pub enum StandardParagraphModifier {
    List,
    ListItem,
    Image,
    AbridgedImage,
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
    AbridgedTodo,
    MultilineTodo,
}

impl StandardParagraphModifier {
    pub fn ordered() -> Vec<Self> {

        //! they must have the compatibility order
        vec![
            Self::MultilineTodo,
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
            Self::AbridgedImage,
            Self::Image,
            Self::CodeBlock,
            Self::CommentBlock,
            Self::ExtendedBlockQuote,
            Self::FocusBlock,
            Self::MathBlock,
            Self::CommonParagraph,
        ]
    }

    pub fn identifier(&self) -> ModifierIdentifier {
        match self {
            Self::Image => String::from("image"),
            Self::CommonParagraph => String::from("common-paragraph"),
            Self::CodeBlock => String::from("code-block"),
            Self::MathBlock => String::from("math-block"),
            Self::ListItem => String::from("list-item"),
            Self::List => String::from("list"),
            Self::ExtendedBlockQuoteLine => String::from("extended-block-quote-line"),
            Self::ExtendedBlockQuote => String::from("extended-block-quote"),
            Self::LineBreakDash => String::from("line-break-dash"),
            Self::LineBreakStar => String::from("line-break-star"),
            Self::LineBreakPlus => String::from("line-break-plus"),
            Self::FocusBlock => String::from("focus-block"),
            Self::AbridgedEmbeddedParagraphStyle => String::from("abridged-embedded-paragraph-style"),
            Self::AbridgedEmbeddedParagraphStyleWithId => String::from("abridged-embedded-paragraph-style-with-id"),
            Self::ParagraphIdentifier => String::from("paragraph-identifier"),
            Self::EmbeddedParagraphStyleWithId => String::from("embedded-paragraph-style-with-id"),
            Self::EmbeddedParagraphStyle => String::from("embedded-paragraph-style"),
            Self::PageBreak => String::from("page-break"),
            Self::AbridgedTodo => String::from("abridged-todo"),
            Self::MultilineTodo => String::from("multiline-todo"),
            // TODO: Self::AbridgedImage => 


            _ => {

                log::warn!("there is NOT a identifier for {:#?}", self);
                String::from("#@§rule-todo#@§")
            }
        }
    }

    // Return the modifier pattern
    pub fn modifier_pattern(&self) -> ModifierPattern {
        match *self {
            Self::Image => String::from(r"!\[([^\]]+)\]\(([^)]+)\)"),

            Self::CommonParagraph => String::from(r#"(?s:(.*?))"#),
            Self::CodeBlock => format!(r"```(\w+){}+(.*?)\n+```", NEW_LINE_PATTERN),
            Self::MathBlock => String::from(r#"\$\$((?s:.+?))\$\$"#),

            Self::ListItem => format!(r#"(?m:^([\t ]*)(-\[\]|-\[ \]|-\[x\]|-\[X\]|-|->|\||\*|\+|--|\d[\.)]?|[a-zA-Z]{{1,8}}[\.)]|&[^;]+;) (.*){})"#, NEW_LINE_PATTERN),
            Self::List => format!(r#"((?:{}+)+)"#, Self::ListItem.modifier_pattern()),
            Self::ExtendedBlockQuoteLine => String::from(r"(?m:^> (.*))"),
            Self::ExtendedBlockQuote => format!(r"(?ms:^> .*?)"),
            Self::LineBreakDash => String::from(r"(?m:^-{3,})"),
            Self::LineBreakStar => String::from(r"(?m:^\*{3,})"),
            Self::LineBreakPlus => String::from(r"(?m:^\+{3,})"),
            Self::FocusBlock => format!(r":::\s(\w+){}(?s:(.*?)){}:::", NEW_LINE_PATTERN, NEW_LINE_PATTERN),
            Self::AbridgedEmbeddedParagraphStyle => String::from(r"\[\[(?sx:(.*?))\]\]\{(.*?)(?s:;(.*?)(?:;(.*?))?)?\}"),
            Self::AbridgedEmbeddedParagraphStyleWithId => format!(r"\[\[(?sx:(.*?))\]\]{}?#([\w-]*){}?\{{(.*?)(?s:;(.*?)(?:;(.*?))?)?\}}", NEW_LINE_PATTERN, NEW_LINE_PATTERN),
            Self::ParagraphIdentifier => format!(r"\[\[(?sx:(.*?))\]\]{}?#([\w-]*)", NEW_LINE_PATTERN),
            Self::EmbeddedParagraphStyleWithId => format!(r"\[\[(?sx:(.*?))\]\]{}?#([\w-]*){}?\{{\{{(?xs:((?:.*?:.*?;?)))\}}\}}", NEW_LINE_PATTERN, NEW_LINE_PATTERN),
            Self::EmbeddedParagraphStyle => String::from(r"\[\[(?sx:(.*?))\]\]\{\{(?xs:((?:.*?:.*?;?)))\}\}"),
            Self::PageBreak => String::from(r"(?m:^#{3,}$)"),
            Self::AbridgedTodo => String::from(r"(?m:^(?i:TODO):?\s(?:(.*?))$)"),
            Self::MultilineTodo => String::from(r"(?i:TODO):(?s:(.*?)):(?i:TODO)"),
            // TODO: Self::AbridgedImage => 
            
            _ => {                                                                  // TODO
                log::warn!("there is NOT a modifier pattern for {:#?}", self);
                String::from(r"RULE TODO")
            }
        }
    }

    pub fn modifier_pattern_with_paragraph_separator(&self) -> String {
        let mp = self.modifier_pattern();

        format!(r"{}{}{}", PARAGRAPH_SEPARATOR_START, mp, PARAGRAPH_SEPARATOR_END)
    }

    pub fn incompatible_modifiers(&self) -> ModifiersBucket {
        match self {

            Self::Image => ModifiersBucket::All,            // TODO: fake... consider alt
            Self::AbridgedImage => ModifiersBucket::All,    // TODO: fake... consider alt
            Self::CodeBlock => ModifiersBucket::All,
            Self::MathBlock => ModifiersBucket::All,
            _ => ModifiersBucket::None
        }
    }
}

impl Into<BaseModifier> for StandardParagraphModifier {
    fn into(self) -> BaseModifier {
        BaseModifier::new(self.identifier(), self.modifier_pattern_with_paragraph_separator(), self.incompatible_modifiers())
    }
}