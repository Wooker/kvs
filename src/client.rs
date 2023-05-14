use serde::Deserialize;
use serde_json::de::{Deserializer, IoRead};
use std::{
    io::{self, BufReader, BufWriter, Write},
    net::{TcpStream, ToSocketAddrs},
};

use crate::command::{Command, GetResponse, RmResponse, SetResponse};

#[derive(Debug)]
pub enum ClientError {
    Bind,
    SerdeError(String),
    NotFound,
    NoArgs,
}

impl From<io::Error> for ClientError {
    fn from(_: io::Error) -> Self {
        ClientError::Bind
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(e: serde_json::Error) -> Self {
        ClientError::SerdeError(e.to_string())
    }
}

pub type ClientResult<T> = Result<T, ClientError>;

pub struct KvsClient {
    reader: Deserializer<IoRead<BufReader<TcpStream>>>,
    writer: BufWriter<TcpStream>,
}

impl KvsClient {
    pub fn new(address: impl ToSocketAddrs) -> ClientResult<Self> {
        let tcp_reader = TcpStream::connect(address)?;
        let tcp_writer = tcp_reader.try_clone()?;

        Ok(KvsClient {
            reader: Deserializer::new(IoRead::new(BufReader::new(tcp_reader))),
            writer: BufWriter::new(tcp_writer),
        })
    }

    pub fn set(&mut self, key: String, val: String) -> ClientResult<()> {
        serde_json::to_writer(&mut self.writer, &Command::Set { key, val })?;
        self.writer.flush()?;

        let response = SetResponse::deserialize(&mut self.reader)?;
        match response {
            SetResponse::Ok(_) => Ok(()),
            SetResponse::Err(e) => Err(ClientError::SerdeError(e.to_string())),
        }
    }

    pub fn get(&mut self, key: String) -> ClientResult<String> {
        serde_json::to_writer(&mut self.writer, &Command::Get { key })?;
        self.writer.flush()?;

        let response = GetResponse::deserialize(&mut self.reader)?;
        match response {
            GetResponse::Ok(val) => Ok(val),
            GetResponse::Err(_) => Err(ClientError::NotFound),
        }
    }

    pub fn rm(&mut self, key: String) -> ClientResult<()> {
        serde_json::to_writer(&mut self.writer, &Command::Rm { key })?;
        self.writer.flush()?;

        let response = RmResponse::deserialize(&mut self.reader)?;
        match response {
            RmResponse::Ok(_) => Ok(()),
            RmResponse::Err(e) => Err(ClientError::SerdeError(e.to_string())),
        }
    }
}
