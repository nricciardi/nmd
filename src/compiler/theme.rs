use serde::{Deserialize, Serialize};


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