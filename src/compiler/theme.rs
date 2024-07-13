use std::{fmt::{Display, Write}, str::FromStr};

use serde::{Deserialize, Serialize};
use thiserror::Error;


#[derive(Error, Debug)]
pub enum ThemeError {
    #[error("unsupported theme: {0}")]
    Unsupported(String)
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Theme {
    Light,
    Dark,
    Scientific,
    Vintage,
    HighContrast,
    None,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Light
    }
}

impl FromStr for Theme {
    type Err = ThemeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "light" => Ok(Self::Light),
            "dark" => Ok(Self::Dark),
            "scientific" => Ok(Self::Scientific),
            "vintage" => Ok(Self::Vintage),
            "high-contrast" => Ok(Self::HighContrast),
            "none" => Ok(Self::None),

            _ => Err(ThemeError::Unsupported(String::from(s))),
        }
    }
}

impl Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Light => "light",
            Self::Dark => "dark",
            Self::Scientific => "scientific",
            Self::Vintage => "vintage",
            Self::HighContrast => "high-contrast",
            Self::None => "none",
        };

        f.write_str(s)
    }
}