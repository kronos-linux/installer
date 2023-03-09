use crate::prelude::*;

#[derive(Debug)]
pub enum Error {
    NoShell(String, String, String),
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
            Self::Generic(s) => error!("Generic error:\n{}", s),
        }

        std::process::exit(1);
    }
}
