use std::{io::{self, Write}, net::{TcpStream, ToSocketAddrs}};

use crate::Command;

#[derive(Debug)]
pub enum ClientError {
    Bind,
    SerdeError(String),
    NoArgs
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
    stream: TcpStream,
}

impl KvsClient {
    pub fn new(address: impl ToSocketAddrs) -> ClientResult<Self> {
        let stream = TcpStream::connect(address)?;

        Ok(KvsClient { stream })
    }

    pub fn send_command(&mut self, command: Command) -> ClientResult<()> {
        self.stream.write(serde_json::to_string(&command)?.as_bytes())?;

        Ok(())
    }
}
