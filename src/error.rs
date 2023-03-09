use crate::prelude::*;

#[derive(Debug)]
pub enum Error {
    NoShell(String, String, String),
    Config(String),
    Usage(String),
    Generic(String),
}

impl Error {
    /// Handle an error by logging the output and exiting with a non zero exit
    /// code
    pub fn handle<T>(&self) -> T {
        match self {
            Self::NoShell(s, cmd, stderr) => error!(
                "NoShell error:\n{}\nCommand: {}\nStderr:\n{}",
                s, cmd, stderr
            ),
            Self::Config(s) => error!("Config error:\n{}", s),
            Self::Usage(s) => error!("Usage error:\n{}", s),
            Self::Generic(s) => error!("Generic error:\n{}", s),
        }

        std::process::exit(1);
    }
}
