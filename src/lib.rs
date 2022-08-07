use std::{io, string::FromUtf8Error};
use serde::{Serialize, Deserialize};

pub mod server;
pub mod client;
pub mod engines;
pub mod thread_pool;


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
            KvsError::IoError(e) => e.to_string(),
            KvsError::SledError(e) => e.to_string(),
            KvsError::SerdeError(e) => e.to_string(),
            KvsError::Utf8Error(e) => e.to_string(),
            _ => "Other".to_string()
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

pub type KvsResult<T> = std::result::Result<T, KvsError>;

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Set(String, String),
    Get(String),
    Rm(String)
}


#[cfg(test)]
mod test {
    use std::{fs::File, env::current_dir};

    use tempfile::TempDir;

    use crate::engines::kvstore::KvStore;

    #[test]
    fn open() {
        let tmp = TempDir::new().unwrap();
        let curr = current_dir().unwrap();

        KvStore::open(curr).unwrap();
        KvStore::open(tmp.path()).unwrap();

    }
}
