use {chrono::NaiveTime, serde::Deserialize};

/// Defines the times when the switch should happen. There is no timezone
/// dependency, just the local time of your PC.
#[derive(Debug, Deserialize)]
pub struct Schedule {
    /// Defines the time to switch to the light mode.
    pub light: NaiveTime,
    /// Defines the time to switch to the dark mode.
    pub dark: NaiveTime,
}
