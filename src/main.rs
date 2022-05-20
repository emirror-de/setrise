#![deny(missing_docs)]
//! Small command line tool that executes configured actions required to change
//! your colorschemes.

mod args;
mod config;
mod schedule;
mod switch_to;
mod theme;

use {
    crate::switch_to::SwitchTo,
    chrono::Local,
    clap::Parser,
    clokwerk::{Scheduler, TimeUnits},
    config::Config,
    std::{thread, time::Duration},
    theme::Theme,
};

const FALLBACK_CONFIG_FILENAME: &str = ".setriserc";

fn setup_scheduler(config: &Config) -> Scheduler {
    let mut scheduler = Scheduler::new();
    for hot_reload in &config.hot_reload {
        let dark_reload = hot_reload.clone();
        let light_reload = hot_reload.clone();
        // execute light
        scheduler
            .every(1.day())
            .at(&config.schedule.light.to_string())
            .run(move || light_reload.switch_to(&Theme::Light));
        // execute dark
        scheduler
            .every(1.day())
            .at(&config.schedule.dark.to_string())
            .run(move || dark_reload.switch_to(&Theme::Dark));
    }
    // commands
    for command in &config.command {
        let dark_reload = command.clone();
        let light_reload = command.clone();
        // execute light
        scheduler
            .every(1.day())
            .at(&config.schedule.light.to_string())
            .run(move || light_reload.switch_to(&Theme::Light));
        // execute dark
        scheduler
            .every(1.day())
            .at(&config.schedule.dark.to_string())
            .run(move || dark_reload.switch_to(&Theme::Dark));
    }
    scheduler
}

fn run_schedules_with_theme(config: &Config, theme: &Theme) {
    for hot_reload in &config.hot_reload {
        hot_reload.switch_to(theme);
    }
    for command in &config.command {
        command.switch_to(theme);
    }
}

fn run_on_startup(config: &Config) {
    log::info!("Determining which theme to use on startup...");
    // set default
    let mut variant = &config.generic.default_theme;
    log::info!("Default startup theme set to {}.", variant);
    // if both light and dark time have passed, set theme to dark
    let dt = Local::now().naive_local();
    let scheduled_light = dt.date().and_time(config.schedule.light);
    let scheduled_dark = dt.date().and_time(config.schedule.dark);
    if (dt > scheduled_light && dt > scheduled_dark)
        || (dt < scheduled_light && dt < scheduled_dark)
    {
        log::info!("Using dark variant.",);
        variant = &Theme::Dark;
    }
    log::info!(
        "Running all schedules on startup with variant {}...",
        variant
    );
    run_schedules_with_theme(config, variant);
    log::info!("...done!");
}

fn main() {
    simple_logger::init_with_env().unwrap();

    let args = args::Args::parse();

    // get home directory
    let home_dir = match dirs::home_dir() {
        Some(h) => h,
        None => {
            log::error!("Could not determine home directory! Exiting...");
            return;
        }
    };

    let config_file = if let Some(f) = args.config_file {
        if f.exists() {
            f
        } else {
            home_dir.join(FALLBACK_CONFIG_FILENAME)
        }
    } else {
        home_dir.join(FALLBACK_CONFIG_FILENAME)
    };

    log::info!("Reading config file at: {}", config_file.display());
    let config_file = match std::fs::read_to_string(config_file) {
        Ok(s) => s,
        Err(e) => {
            log::error!(
                "Could not read config file with error:\n{:#?}\nExiting...",
                e
            );
            return;
        }
    };
    let config: Config = match toml::from_str(&config_file) {
        Ok(c) => c,
        Err(e) => {
            log::error!(
                "Could not parse config file with error:\n{:#?}\nExiting...",
                e
            );
            return;
        }
    };
    log::info!("{:#?}", config);
    run_on_startup(&config);
    let mut scheduler = setup_scheduler(&config);
    log::info!(
        "Starting event loop that wakes up every {} seconds...",
        config.generic.event_loop_seconds
    );
    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_secs(config.generic.event_loop_seconds));
    }
}
