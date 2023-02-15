use miette::Diagnostic;
use simple_error::SimpleError;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("PassManager::run on module `{0}` failed")]
    PassManagerRunFailure(String),
    #[error(transparent)]
    SimpleError(#[from] SimpleError),
    #[error("Option was none!")]
    IsNone,
}

impl Error {
    pub fn simple(msg: impl Into<String>) -> Error {
        Error::SimpleError(SimpleError::new(msg))
    }
}

