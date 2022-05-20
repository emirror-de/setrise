use {clap::Parser, std::path::PathBuf};

/// Small, infinitely running, application to adjust your colorschemes based on a configured daytime.
///
/// If no config file is given, ~/.setriserc is being used.
#[derive(Parser, Debug)]
#[clap(author, version)]
pub struct Args {
    /// The configuration TOML file.
    #[clap(short)]
    pub config_file: Option<PathBuf>,
}
