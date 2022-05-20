use {
    crate::{switch_to::SwitchTo, theme::Theme},
    serde::Deserialize,
    std::path::PathBuf,
};

/// Configuration for applications that reload config files during runtime.
/// The given files will be copied to the target.
#[derive(Debug, Clone, Deserialize)]
pub struct HotReload {
    name: String,
    target: PathBuf,
    dark: PathBuf,
    light: PathBuf,
}

impl SwitchTo for HotReload {
    fn switch_to(&self, theme: &Theme) {
        log::info!("Switching to {} variant for {}...", theme, &self.name,);
        let source = match theme {
            Theme::Light => &self.light,
            Theme::Dark => &self.dark,
        };
        let source = PathBuf::from(
            shellexpand::tilde(
                source.to_str().expect("Could not convert PathBuf to str"),
            )
            .into_owned(),
        );
        let target = PathBuf::from(
            shellexpand::tilde(
                self.target
                    .to_str()
                    .expect("Could not convert PathBuf to str"),
            )
            .into_owned(),
        );

        if !source.exists() {
            log::info!(
                "{}: Source file {} does not exist!",
                &self.name,
                source.display()
            );
            return;
        }

        if let Err(e) = std::fs::copy(&source, &target) {
            log::error!(
                "Could not copy file from {} to {}. Error:\n{:#?}",
                source.display(),
                target.display(),
                e
            );
        };
        log::info!("...done!");
    }
}
