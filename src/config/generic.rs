use {crate::Theme, serde::Deserialize};

/// Contains generic settings.
#[derive(Debug, Deserialize)]
pub struct Generic {
    /// Seconds that the event loop will sleep.
    pub event_loop_seconds: u64,
    /// Default startup theme.
    pub default_theme: Theme,
}
