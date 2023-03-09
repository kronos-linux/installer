use crate::prelude::*;

#[derive(Debug)]
pub enum Error {
    Generic(String),
}

impl Error {
    pub fn handle<T>(&self) -> T {
        match self {
            Self::Generic(s) => error!("Generic error:\n{}", s),
        }

        std::process::exit(1);
    }
}
