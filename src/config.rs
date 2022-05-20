use {crate::schedule::Schedule, serde::Deserialize};

mod command;
mod generic;
mod hot_reload;

pub use {command::Command, generic::Generic, hot_reload::HotReload};

/// Main representation of a config file.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub generic: Generic,
    /// Contains the definition when themes to be switched.
    pub schedule: Schedule,
    /// Applications that have build in hot reloading.
    pub hot_reload: Vec<HotReload>,
    /// Custom commands to run for desired themes.
    pub command: Vec<Command>,
}
