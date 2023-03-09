use crate::prelude::*;
use serde::de::Deserialize;

/// Setup the logger for the installer
pub fn setup_logger() {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .parse_env("LOG_LEVEL")
        .write_style(env_logger::WriteStyle::Always)
        .target(env_logger::Target::Stdout)
        .init();
}

/// Return a config from a target string
pub fn generate_config(target: &str) -> Config {
    let c = match Config::builder()
        .add_source(config::File::with_name(target))
        .build()
    {
        Ok(g) => g,
        Err(e) => Error::Config(format!("Could not build config:\n{}", e)).handle(),
    };
    c
}

/// Generate a target string from the supplied argument
pub fn config_target() -> String {
    let argv: Vec<String> = std::env::args().collect();

    match argv.len() {
        2 => argv[1].clone(),
        _ => {
            let estr1 = "Usage:\n ./installer [path to config.toml]";
            Error::Usage(estr1.to_string()).handle()
        }
    }
}

/// Run a shell command and stop the installation if there is an error
pub fn shrun(cmd: &ShellCommand) -> String {
    let cmd_name = String::from(cmd.command());

    match cmd.run() {
        Ok(s) => s,
        Err(e) => {
            let estr = "Shell command exited with a non-zero exit code".into();
            Error::NoShell(estr, cmd_name, e).handle()
        }
    }
}

/// Add a value to the installation configuration
pub fn add_value<T: Into<config::Value>>(c: Config, key: impl Into<String>, val: T) -> Config {
    Config::builder()
        .add_source(c)
        .set_override(key.into(), val)
        .expect("Adding override to configuration failed")
        .build()
        .expect("Failed to build config with new value")
}

/// Get value from installation configuration
pub fn get_value<T: Deserialize<'static>>(c: &Config, key: impl Into<String>) -> T {
    match c.get::<T>(&key.into()) {
        Ok(v) => v,
        Err(e) => Error::Config(e.to_string()).handle(),
    }
}
