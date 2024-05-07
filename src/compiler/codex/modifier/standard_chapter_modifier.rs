use regex::Regex;

use crate::compiler::codex::modifier::constants::HEADING_ANNOTATIONS_PATTERN;

use super::base_modifier::BaseModifier;
use super::constants::MAX_HEADING_LEVEL;
use super::modifiers_bucket::ModifiersBucket;
use super::ModifierIdentifier;

#[derive(Debug, PartialEq, Clone)]
pub enum StandardChapterModifier {

    HeadingGeneralCompactVersion(u32),
    HeadingGeneralExtendedVersion(u32),
    MinorHeading,
    MajorHeading,
    SameHeading,

}

impl StandardChapterModifier {
    pub fn ordered() -> Vec<Self> {
        let mut heading_modifiers: Vec<Self> = vec![Self::MinorHeading, Self::MajorHeading, Self::SameHeading];

        for i in (1..=MAX_HEADING_LEVEL).rev() {
            heading_modifiers.push(Self::HeadingGeneralExtendedVersion(i));
            heading_modifiers.push(Self::HeadingGeneralCompactVersion(i));
        }

        heading_modifiers
    }

    pub fn heading_level(content: &str) -> Option<u32> {
        let heading_modifiers = Self::ordered();

        for heading_modifier in heading_modifiers {
            let regex = Regex::new(&heading_modifier.modifier_pattern()).unwrap();

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
            StandardChapterModifier::MinorHeading => String::from("minor-heading"),
            StandardChapterModifier::MajorHeading => String::from("major-heading"),
            StandardChapterModifier::SameHeading => String::from("same-heading"),
        }
    }
    
    pub fn modifier_pattern(&self) -> String {
        let specific_pattern = match *self {
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
            StandardChapterModifier::MinorHeading => String::from(r"(?m:^#-\s+(.*))"),
            StandardChapterModifier::MajorHeading => String::from(r"(?m:^#\+\s+(.*))"),
            StandardChapterModifier::SameHeading => String::from(r"(?m:^#=\s+(.*))"),
        };

        format!("{}{}", specific_pattern, HEADING_ANNOTATIONS_PATTERN)
    }

    pub fn incompatible_modifiers(&self) -> ModifiersBucket {
        ModifiersBucket::None
    }
}

impl Into<BaseModifier> for StandardChapterModifier {
    fn into(self) -> BaseModifier {
        BaseModifier::new(self.identifier(), self.modifier_pattern(), self.incompatible_modifiers())
    }
}