use std::{collections::HashMap, path::PathBuf, fs::{File, OpenOptions}, io::{self, BufReader, BufRead, Write}};
use crate::{KvsEngine, Command, KvsResult, KvsError};


pub struct KvStore {
    path: PathBuf,
    map: HashMap<String, String>,
    index: Option<PathBuf>,
}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> KvsResult<Self>
    {
        let mut path: PathBuf = path.into();

        if path.is_dir() {
            path.push("kvs-1.log");
        }

        let mut map = HashMap::<String, String>::new();

        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .create(true)
            .open(path.clone())?;


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

impl KvsEngine for KvStore {
    fn set(&mut self, key: String, val: String) -> KvsResult<()> {
        let mut f = OpenOptions::new()
            .write(true)
            .append(true)
            .open(self.index.as_ref().unwrap())?;
        writeln!(f, "{}", serde_json::to_value(Command::Set(key.clone(), val.clone()))?)?;

        self.map.insert(key, val);
        Ok(())
    }

    fn get(&self, key: String) -> KvsResult<String> {
        match self.map.get(&key).cloned() {
            Some(val) => Ok(val),
            None => Err(KvsError::NotFound) }
    }

    fn remove(&mut self, key: String) -> KvsResult<()> {
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

}
