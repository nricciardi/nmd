use std::str::FromStr;

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
            "none" => Ok(Self::None),

            _ => Err(ThemeError::Unsupported(String::from(s))),
        }
    }
}