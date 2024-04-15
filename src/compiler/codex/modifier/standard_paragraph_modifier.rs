use super::{base_modifier::BaseModifier, modifiers_bucket::ModifiersBucket, Modifier, ModifierIdentifier, ModifierPattern};


pub const PARAGRAPH_SEPARATOR_START: &str = r"(?m:^[ \t]*\n)+";
pub const PARAGRAPH_SEPARATOR_END: &str = r"(?m:^[ \t]*\n){1}";


#[derive(Debug, PartialEq, Clone)]
pub enum StandardParagraphModifier {
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
    AbridgedTodo,
}

impl StandardParagraphModifier {
    pub fn ordered() -> Vec<Self> {

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


            _ => String::from("#@§rule-todo#@§"),
        }
    }

    pub fn modifier_pattern(&self) -> ModifierPattern {
        match *self {
            Self::Image => String::from(r"!\[([^\]]+)\]\(([^)]+)\)"),

            Self::CommonParagraph => String::from(r#"(?s:(.*?))"#),       // TODO
            Self::CodeBlock => String::from(r"```(\w+)\n+(.*?)\n+```"),
            Self::MathBlock => String::from(r#"\$\$((?s:.+?))\$\$"#),

            Self::ListItem => String::from(r#"(?m:^([\t ]*)(-\[\]|-\[ \]|-\[x\]|-\[X\]|-|->|\||\*|\+|--|\d[\.)]?|[a-zA-Z]{1,8}[\.)]|&[^;]+;) (.*)\n)"#),
            Self::List => format!(r#"((?:{}+)+)"#, Self::ListItem.modifier_pattern()),
            Self::ExtendedBlockQuoteLine => String::from(r"(?m:^> (.*))"),
            Self::ExtendedBlockQuote => format!(r"({}){}({})?", Self::ExtendedBlockQuoteLine.modifier_pattern(), String::from(r"\n(?:(?mx:^> .*\n)*)"), Self::ExtendedBlockQuoteLine.modifier_pattern()),
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
    
    pub fn searching_pattern(&self) -> String {
        let mp = self.modifier_pattern();

        format!(r"{}{}{}", PARAGRAPH_SEPARATOR_START, mp, PARAGRAPH_SEPARATOR_END)
    }

    pub fn incompatible_modifiers(&self) -> ModifiersBucket {
        match self {

            Self::Image => ModifiersBucket::All,
            Self::CodeBlock => ModifiersBucket::All,
            Self::MathBlock => ModifiersBucket::All,
            _ => ModifiersBucket::None
        }
    }
}

impl Into<BaseModifier> for StandardParagraphModifier {
    fn into(self) -> BaseModifier {
        BaseModifier::new(self.identifier(), self.searching_pattern(), self.incompatible_modifiers())
    }
}