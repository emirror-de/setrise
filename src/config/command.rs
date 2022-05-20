use {
    crate::{SwitchTo, Theme},
    serde::Deserialize,
    std::process::Command as StdCommand,
};

/// Configuration for custom commands that set the desired theme.
#[derive(Debug, Clone, Deserialize)]
pub struct Command {
    name: String,
    dark: String,
    light: String,
}

impl SwitchTo for Command {
    fn switch_to(&self, theme: &Theme) {
        let command_to_run = match theme {
            Theme::Light => &self.light,
            Theme::Dark => &self.dark,
        };
        log::info!("Running command for {}: {}", &self.name, &command_to_run);
        let output = if cfg!(target_os = "windows") {
            StdCommand::new("cmd")
                .args(["/C", command_to_run])
                .output()
                .expect("failed to execute process")
        } else {
            StdCommand::new("sh")
                .arg("-c")
                .arg(command_to_run)
                .output()
                .expect("failed to execute process")
        };

        log::info!("{}", output.status);
    }
}
