use super::{base_modifier::BaseModifier, modifiers_bucket::ModifiersBucket, Modifier, ModifierIdentifier};



#[derive(Debug, PartialEq, Clone)]
pub enum ParagraphModifier {
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

pub type ParagraphType = ParagraphModifier;

impl ParagraphModifier {
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
            Self::Image => String::from(r"!\[([^\]]+)\]\(([^)]+)\)"),
            Self::CommonParagraph => String::from(r#"([\s\S]*?)"#),
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

            _ => String::from("#@§rule-todo#@§"),
        }
    }
    
    pub fn search_pattern(&self) -> String {
        match *self {
            Self::Image => String::from(r"!\[([^\]]+)\]\(([^)]+)\)"),

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

    pub fn incompatible_modifiers(&self) -> ModifiersBucket {
        match self {

            Self::Image => ModifiersBucket::All,
            Self::CodeBlock => ModifiersBucket::All,
            Self::MathBlock => ModifiersBucket::All,
            _ => ModifiersBucket::None
        }
    }
}

impl Into<BaseModifier> for ParagraphModifier {
    fn into(self) -> BaseModifier {
        BaseModifier::new(self.identifier(), self.search_pattern(), self.incompatible_modifiers())
    }
}