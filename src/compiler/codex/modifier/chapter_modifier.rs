use regex::Regex;

use super::base_modifier::BaseModifier;
use super::{modifiers_bucket::ModifiersBucket, Modifier};
use super::{ModifierIdentifier, MAX_HEADING_LEVEL};

#[derive(Debug, PartialEq, Clone)]
pub enum ChapterModifier {

    HeadingGeneralCompactVersion(u32),
    HeadingGeneralExtendedVersion(u32),

}

impl ChapterModifier {
    pub fn ordered() -> Vec<Self> {
        let mut heading_modifiers: Vec<Self> = Vec::new();

        for i in (1..=MAX_HEADING_LEVEL).rev() {
            heading_modifiers.push(Self::HeadingGeneralExtendedVersion(i));
            heading_modifiers.push(Self::HeadingGeneralCompactVersion(i));
        }

        heading_modifiers
    }

    pub fn heading_level(content: &str) -> Option<u32> {
        let heading_modifiers = Self::ordered();

        for heading_modifier in heading_modifiers {
            let regex = Regex::new(&heading_modifier.searching_pattern()).unwrap();

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

    pub fn str_is_heading(content: &str) -> bool {
        Self::heading_level(content).is_some()
    }

    pub fn identifier(&self) -> ModifierIdentifier {
        match *self {
            Self::HeadingGeneralExtendedVersion(level) => {

                if level == 0 || level > MAX_HEADING_LEVEL {
                    panic!("{level} is an invalid heading level.")
                }

                format!(r"heading-{}-extended-version", level)
            },
            Self::HeadingGeneralCompactVersion(level) => {

                if level == 0 || level > MAX_HEADING_LEVEL {
                    panic!("{level} is an invalid heading level.")
                }

                format!(r"heading-{}-compact-version", level)
            },
        }
    }
    
    pub fn searching_pattern(&self) -> String {
        match *self {
            Self::HeadingGeneralExtendedVersion(level) => {

                if level == 0 || level > MAX_HEADING_LEVEL {
                    panic!("{level} is an invalid heading level.")
                }

                format!(r"(?m:^#{{{}}}\s+(.*))", level)
            },
            Self::HeadingGeneralCompactVersion(level) => {

                if level == 0 || level > MAX_HEADING_LEVEL {
                    panic!("{level} is an invalid heading level.")
                }

                format!(r"(?m:^#({})\s+(.*))", level)
            },
        }
    }

    pub fn incompatible_modifiers(&self) -> ModifiersBucket {
        ModifiersBucket::None
    }
}

impl Into<BaseModifier> for ChapterModifier {
    fn into(self) -> BaseModifier {
        BaseModifier::new(self.identifier(), self.searching_pattern(), self.incompatible_modifiers())
    }
}