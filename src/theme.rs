use serde::Deserialize;

/// Defines the theme variant.
#[derive(Debug, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
