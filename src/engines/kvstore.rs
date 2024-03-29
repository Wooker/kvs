use crate::{command::Command, engines::KvsEngine, KvsError, KvsResult};
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    sync::{Arc, RwLock},
};

#[derive(Clone)]
pub struct KvStore(Arc<RwLock<RwLockKvStore>>);

#[derive(Clone)]
pub struct RwLockKvStore {
    map: HashMap<String, String>,
    path: PathBuf,
}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> KvsResult<Self> {
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
                Command::Set { key, val } => {
                    map.insert(key, val);
                }
                Command::Rm { key } => {
                    map.remove(&key);
                }
                _ => {}
            }
        }

        let index = path.clone();

        // Remove file name from path
        path.pop();

        Ok(KvStore(Arc::new(RwLock::new(RwLockKvStore {
            map,
            path: index,
        }))))
    }
}

impl KvsEngine for KvStore {
    fn set(&mut self, key: String, val: String) -> KvsResult<()> {
        let mut kvstore = self.0.write().unwrap();
        let mut f = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&kvstore.path)?;
        writeln!(
            f,
            "{}",
            serde_json::to_value(Command::Set {
                key: key.clone(),
                val: val.clone()
            })?
        )?;

        kvstore.map.insert(key, val);
        Ok(())
    }

    fn get(&self, key: String) -> KvsResult<String> {
        let kvstore = self.0.read().unwrap();
        match kvstore.map.get(&key).cloned() {
            Some(val) => Ok(val),
            None => Err(KvsError::NotFound),
        }
    }

    fn remove(&mut self, key: String) -> KvsResult<()> {
        let mut kvstore = self.0.write().unwrap();
        let mut f = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&kvstore.path)?;
        writeln!(
            f,
            "{}",
            serde_json::to_value(Command::Rm { key: key.clone() })?
        )?;

        match kvstore.map.remove(&key) {
            Some(_) => Ok(()),
            None => Err(KvsError::RemoveError("Key not found".to_string())),
        }
    }
}
