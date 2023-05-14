use std::{io, string::FromUtf8Error};

pub mod client;
pub mod command;
pub mod engines;
pub mod server;
pub mod thread_pool;

pub type KvsResult<T> = std::result::Result<T, KvsError>;

#[derive(Debug)]
pub enum KvsError {
    NotFound,
    SetError,
    RemoveError(String),
    NoArgs,
    IoError(String),
    SerdeError(String),
    SledError(String),
    Utf8Error(String),
    NotImplemented,
}

impl ToString for KvsError {
    fn to_string(&self) -> String {
        match self {
            KvsError::IoError(e)
            | KvsError::SledError(e)
            | KvsError::SerdeError(e)
            | KvsError::Utf8Error(e) => e.to_string(),
            _ => "Other".to_string(),
        }
    }
}

impl From<io::Error> for KvsError {
    fn from(e: io::Error) -> Self {
        KvsError::IoError(e.to_string())
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(e: serde_json::Error) -> Self {
        KvsError::SerdeError(e.to_string())
    }
}

impl From<sled::Error> for KvsError {
    fn from(e: sled::Error) -> Self {
        KvsError::SledError(e.to_string())
    }
}

impl From<FromUtf8Error> for KvsError {
    fn from(e: FromUtf8Error) -> Self {
        KvsError::Utf8Error(e.to_string())
    }
}
