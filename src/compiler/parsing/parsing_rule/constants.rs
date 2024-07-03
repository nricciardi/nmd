use once_cell::sync::Lazy;
use regex::Regex;

use crate::compiler::codex::modifier::constants::NEW_LINE;



pub static DOUBLE_NEW_LINE_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(&format!("{}{{2}}", NEW_LINE)).unwrap());


pub const SPACE_TAB_EQUIVALENCE: &str = r"   ";