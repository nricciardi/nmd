use crate::compiler::parsable::codex::Modifier;

pub struct ChapterHeading {
    raw_heading: String,
    level: u32
}

impl ChapterHeading {
    pub fn unrestricted_new(raw_heading: String, level: u32) -> Self {
        Self {
            raw_heading,
            level
        }
    }
}

impl TryFrom<String> for ChapterHeading {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let is_heading = Modifier::is_heading(&value);

        if is_heading.is_none() {
            return Err(format!("{} is not an heading", value));
        }

        Ok(Self {
            raw_heading: value,
            level: is_heading.unwrap()
        })
    }
}

impl Clone for ChapterHeading {
    fn clone(&self) -> Self {
        Self { raw_heading: self.raw_heading.clone(), level: self.level.clone() }
    }
}