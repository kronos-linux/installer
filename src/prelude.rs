pub use crate::error::*;

pub type Result<T> = core::result::Result<T, Error>;

pub use log::{debug, error, info, trace, warn};

pub use crate::utils::*;
pub use noshell::ShellCommand;

pub use config::Config;
