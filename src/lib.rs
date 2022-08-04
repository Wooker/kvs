use std::{collections::HashMap, path::{Path, PathBuf}, fs::{File, OpenOptions}, io::{BufReader, self, BufRead, Write, BufWriter}, env::current_dir};

use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Debug)]
pub enum KvsError {
    NotFound,
    SetError,
    RemoveError,
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
    map: HashMap<String, String>,
    index: Option<PathBuf>,
}

impl KvStore {
    pub fn new() -> Self {
        KvStore {
            map: HashMap::new(),
            index: None
        }
    }

    pub fn set(&mut self, key: String, val: String) -> Result<()> {
        let mut f = OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.index.as_ref().unwrap())?;
        writeln!(f, "{}", serde_json::to_value(Command::Set(key.clone(), val.clone()))?)?;

        self.map.insert(key, val);
        Ok(())
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        match self.map.get(&key).cloned() {
            Some(val) => Ok(Some(val)),
            None => Err(KvsError::NotFound)
        }
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        let mut f = OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.index.as_ref().unwrap())?;
        writeln!(f, "{}", serde_json::to_value(Command::Rm(key.clone()))?)?;

        match self.map.remove(&key) {
            Some(_) => Ok(()),
            None => Err(KvsError::SerdeError("Key not found".to_string()))
        }
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<Self>
    {
        let mut path = PathBuf::from(path.into());
        path.push("kvs-1.log");

        let mut kvs = KvStore::new();
        kvs.index = Some(path.clone());

        if !path.is_file() {
            Ok(kvs)
        } else {
            let f = File::open(path)?;

            let rdr = BufReader::new(f);
            for l in rdr.lines() {
                let val = serde_json::from_str(&l?)?;
                let command: Command = serde_json::from_value(val)?;

                match command {
                    Command::Set(key, val) => {
                        kvs.map.insert(key, val);
                    }
                    Command::Rm(key) => {
                        kvs.map.remove(&key);
                    }
                    _ => {}
                }
            }

            Ok(kvs)
        }
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
