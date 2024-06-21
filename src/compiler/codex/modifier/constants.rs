pub const CHAPTER_TAGS_PATTERN: &str = r"(?:\n@(.*))*";
pub const CHAPTER_STYLE_PATTERN: &str = r"(\n\{(?s:(.*))\})?";
pub const NEW_LINE_PATTERN: &str = r"[[:cntrl:]]";
pub const IDENTIFIER_PATTERN: &str = r"#([\w-]+)";

pub const MAX_HEADING_LEVEL: u32 = 6;