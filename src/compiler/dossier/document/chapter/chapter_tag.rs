use std::str::FromStr;

use getset::{Getters, Setters};
use once_cell::sync::Lazy;
use regex::Regex;

static FROM_STR_PATTERN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"@(\w+) (.*)").unwrap());


#[derive(Debug, Clone, Getters, Setters)]
pub struct ChapterTag {

    #[getset(get = "pub", set = "pub")]
    key: ChapterTagKey,

    #[getset(get = "pub", set = "pub")]
    value: Option<String>
}

#[derive(Debug, Clone)]
pub enum ChapterTagKey {
    Id,
    Author,
    Date,
    Intent,
    Style,
    StyleClass,
    None
}

impl FromStr for ChapterTagKey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "id" => Ok(Self::Id),
            "author" => Ok(Self::Author),
            "date" => Ok(Self::Date),
            "intent" => Ok(Self::Intent),
            "style" => Ok(Self::Style),
            "styleclass" => Ok(Self::StyleClass),

            _ => Err(format!("chapter key '{}' not found", s))
        }
    }
}

impl Default for ChapterTag {
    fn default() -> Self {
        Self { key: ChapterTagKey::None, value: None }
    }
}


impl FromStr for ChapterTag {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let captures = FROM_STR_PATTERN_REGEX.captures(s);

        if let Some(captures) = captures {

            if let Some(key) = captures.get(1) {

                let mut chapter_tag = ChapterTag::default();
                chapter_tag.set_key(ChapterTagKey::from_str(key.as_str())?);

                if let Some(value) = captures.get(2) {
                    chapter_tag.set_value(Some(value.as_str().to_string()));
                }

                return Ok(chapter_tag)
            }
        }
        
        Err(format!("{} is not a valid tag", s))
    }
}