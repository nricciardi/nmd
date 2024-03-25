use std::{num::ParseIntError, str::FromStr};

use super::chapter_builder::ChapterBuilderError;


#[derive(Debug, Clone)]
pub enum HeadingLevel {
    PrecedentePlusOne,
    PrecedenteMinusOne,
    Numerical(u32)
}

impl FromStr for HeadingLevel {
    type Err = ParseIntError;     // TODO

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq("+") {
            return Ok(Self::PrecedentePlusOne)
        }

        if s.eq("-") {
            return Ok(Self::PrecedenteMinusOne)
        }

        match s.parse::<u32>() {
            Ok(n) => Ok(Self::Numerical(n)),
            Err(e) => Err(e)
        }
    }
}


#[derive(Debug, Clone)]
pub struct Heading {
    level: HeadingLevel,
    title: String
}

impl Heading {
    pub fn new(level: HeadingLevel, title: String) -> Self {
        Self {
            level,
            title
        }
    }

    pub fn level(&self) -> &HeadingLevel {
        &self.level
    }

    pub fn title(&self) -> &String {
        &self.title
    }
}

