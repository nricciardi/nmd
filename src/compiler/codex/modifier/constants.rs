pub const CHAPTER_TAGS_PATTERN: &str = r"(?:\r?\n@(.*))*";
pub const CHAPTER_STYLE_PATTERN: &str = r"(\r?\n\{(?s:(.*))\})?";
pub const IDENTIFIER_PATTERN: &str = r"#([\w-]+)";

pub const MAX_HEADING_LEVEL: u32 = 6;

#[cfg(windows)]
pub const NEW_LINE: &str = r"\r\n";

#[cfg(not(windows))]
pub const NEW_LINE: &str = r"\n";