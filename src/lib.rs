use std::{collections::HashMap, path::{Path, PathBuf}, fs::{File, OpenOptions}, io::{BufReader, self, BufRead, Write, BufWriter}, env::current_dir};

use serde::{Serialize, Deserialize};
use serde_json::json;

pub mod server;
pub mod client;

#[derive(Debug)]
pub enum KvsError {
    NotFound,
    SetError,
    RemoveError(String),
    NoArgs,
    IoError,
    SerdeError(String),
    NotImplemented,
}

impl From<io::Error> for KvsError {
    fn from(_: io::Error) -> Self {
        KvsError::IoError
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(e: serde_json::Error) -> Self {
        KvsError::SerdeError(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, KvsError>;

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Set(String, String),
    Get(String),
    Rm(String)
}

#[derive(Debug)]
pub struct KvStore {
    path: PathBuf,
    map: HashMap<String, String>,
    index: Option<PathBuf>,
}

impl KvStore {
    pub fn set(&mut self, key: String, val: String) -> Result<()> {
        let mut f = OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.index.as_ref().unwrap())?;
        writeln!(f, "{}", serde_json::to_value(Command::Set(key.clone(), val.clone()))?)?;

        self.map.insert(key, val);
        Ok(())
    }

    pub fn get(&self, key: String) -> Result<String> {
        match self.map.get(&key).cloned() {
            Some(val) => Ok(val),
            None => Err(KvsError::NotFound) }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        let mut f = OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.index.as_ref().unwrap())?;
        writeln!(f, "{}", serde_json::to_value(Command::Rm(key.clone()))?)?;

        match self.map.remove(&key) {
            Some(_) => Ok(()),
            None => Err(KvsError::RemoveError("Key not found".to_string()))
        }
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<Self>
    {
        let mut path = PathBuf::from(path.into());
        path.push("kvs-1.log");

        let mut map = HashMap::<String, String>::new();

        let f = File::open(path.clone())?;

        let rdr = BufReader::new(f);
        for l in rdr.lines() {
            let val = serde_json::from_str(&l?)?;
            let command: Command = serde_json::from_value(val)?;

            match command {
                Command::Set(key, val) => {
                    map.insert(key, val);
                }
                Command::Rm(key) => {
                    map.remove(&key);
                }
                _ => {}
            }
        }

        let index = path.clone();

        // Remove file name from path
        path.pop();

        Ok(KvStore {
            map,
            path,
            index: Some(index),
        })
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;

    use tempfile::TempDir;

    #[test]
    fn open() {
        let tmp = TempDir::new().unwrap();

        let f = File::open(tmp).unwrap();
        let f2 = File::open("/tmp/").unwrap();
        dbg!(&f);
        dbg!(&f2);

    }
}
