use std::{net::{TcpListener, ToSocketAddrs}, io::{self, Read}, path::PathBuf};

use crate::{KvsEngine, Command, engines::kvstore::KvStore, KvsError};

#[derive(Debug)]
pub enum ServerError {
    Bind,
    SerdeError(String),
    KvsError(String),
    Other
}

impl From<io::Error> for ServerError {
    fn from(_: io::Error) -> Self {
        ServerError::Bind
    }
}

impl From<serde_json::Error> for ServerError {
    fn from(e: serde_json::Error) -> Self {
        ServerError::SerdeError(e.to_string())
    }
}

impl From<KvsError> for ServerError {
    fn from(e: KvsError) -> Self {
        ServerError::KvsError(e.to_string())
    }
}

pub type ServerResult<T> = Result<T, ServerError>;

pub struct KvsServer {
    listener: TcpListener,
    store: KvStore,
}

impl KvsServer {
    pub fn new(address: impl ToSocketAddrs, dir: impl Into<PathBuf>) -> ServerResult<Self> {
        let listener = TcpListener::bind(address)?;

        Ok(KvsServer { listener, store: KvStore::open(dir)? })
    }

    pub fn handle(&mut self) -> ServerResult<()>{
        for stream in self.listener.incoming() {
            let mut s = String::new();
            let len = stream.unwrap().read_to_string(&mut s)?;

            if len != 0 {
                let command = serde_json::from_str::<Command>(s.as_str())?;
                match command {
                    Command::Set(key, val) => {
                        println!("Set {}: {}", key, val);
                        self.store.set(key, val)?;
                    }
                    Command::Get(key) => {
                        match self.store.get(key) {
                            Ok(val) => println!("Get {:?}", val),
                            Err(e) => println!("{:?}", e),
                        }
                    }
                    Command::Rm(key) => {
                        println!("Rm {}", key);
                        self.store.remove(key)?;
                    }
                }
            }
        }

        Ok(())
    }
}
