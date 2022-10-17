use std::{error, fmt};
use std::fmt::{Formatter};

#[derive(Debug)]
pub enum Error {
    /// reqwest error
    ReqwestError(reqwest::Error),
    /// api invoke error
    ApiError(String),
    /// io error
    IoError(std::io::Error),
}

/// for better print of Error
impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::ReqwestError(e) => { write!(f, "reqwest error: {}", e) }
            Error::ApiError(e) => { write!(f, "api invoke error: {}", e) }
            Error::IoError(e) => { write!(f, "io error: {}", e) }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::ReqwestError(e) => { Some(e) }
            Error::ApiError(_) => None,
            Error::IoError(e) => { Some(e) }
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::ReqwestError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}