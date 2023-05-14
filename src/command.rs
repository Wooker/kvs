use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Set { key: String, val: String },
    Get { key: String },
    Rm { key: String },
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SetResponse {
    Ok(()),
    Err(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum GetResponse {
    Ok(String),
    Err(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RmResponse {
    Ok(()),
    Err(String),
}
