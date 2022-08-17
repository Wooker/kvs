use std::{net::{TcpListener, ToSocketAddrs, TcpStream}, io::{self, Read, Write, BufWriter, BufReader}, path::PathBuf};

use serde::{Serialize, Deserialize};
use serde_json::Deserializer;

use crate::{engines::KvsEngine, command::{Command, GetResponse, SetResponse, RmResponse}, engines::kvstore::KvStore, KvsError, thread_pool::ThreadPool};

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerError {
    Bind,
    SerdeError(String),
    KvsError(String),
    Other(String)
}

impl From<io::Error> for ServerError {
    fn from(_: io::Error) -> Self {
        ServerError::Bind
    }
}

impl From<&io::Error> for ServerError {
    fn from(_: &io::Error) -> Self {
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

impl ToString for ServerError {
    fn to_string(&self) -> String {
        match self {
            ServerError::KvsError(e) => format!("KvsError: {}", e.to_string()),
            ServerError::SerdeError(e) => format!("SerdeError: {}", e.to_string()),
            ServerError::Other(e) => format!("Other {}", e.to_string()),
            ServerError::Bind => "Bind error".to_string()
        }
    }
}

pub type ServerResult<T> = Result<T, ServerError>;

pub struct KvsServer<E: KvsEngine, P: ThreadPool> {
    engine: E,
    pool: P,
}

impl<E: KvsEngine, P: ThreadPool> KvsServer<E, P> {
    pub fn new(engine: E, pool: P) -> ServerResult<Self> {
        Ok(KvsServer {
            engine,
            pool,
        })
    }

    pub fn run(&mut self, address: impl ToSocketAddrs) -> ServerResult<()>{
        let listener = TcpListener::bind(address)?;

        for stream in listener.incoming() {
            let engine = self.engine.clone();
            self.pool.spawn(move || match stream {
                Ok(stream) => {
                    if let Err(e) = handle(engine, stream) {
                        println!("Error: {}", e.to_string());
                    }
                }
                Err(e) => { 
                    println!("Tcp error: {}", e.to_string());
                }
            });
        }

        Ok(())
    }
}

fn handle<E: KvsEngine>(mut engine: E, stream: TcpStream) -> ServerResult<()>{
    let reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);
    let req_seq = Deserializer::from_reader(reader).into_iter::<Command>();

    // Process the command
    for req in req_seq {
        let req = req?;

        match req {
            Command::Set{ key, val } => {
                println!("Set {}: {}", key, val);
                engine.set(key, val)?;

                serde_json::to_writer(&mut writer, &SetResponse::Ok(()))?;
                writer.flush()?;
            }
            Command::Get{ key } => {
                match engine.get(key) {
                    Ok(val) => {
                        println!("Get {:?}", val);

                        serde_json::to_writer(&mut writer, &GetResponse::Ok(val))?;
                        writer.flush()?;
                    }
                    Err(e) => {
                        println!("{:?}", e);

                        serde_json::to_writer(&mut writer, &GetResponse::Err(String::from("Not found")))?;
                        writer.flush()?;
                    }
                }

            }
            Command::Rm{ key } => {
                println!("Rm {}", key);
                engine.remove(key)?;

                serde_json::to_writer(&mut writer, &RmResponse::Ok(()))?;
                writer.flush()?;
            }
        }
    }

    Ok(())
}
