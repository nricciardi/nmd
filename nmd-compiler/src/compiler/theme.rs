use serde::Deserialize;


#[derive(Debug, Clone, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Light
    }
}