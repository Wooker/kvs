use crate::KvsResult;

pub mod kvstore;
pub mod sled;

pub trait KvsEngine {
    fn set(&mut self, key: String, val: String) -> KvsResult<()>;
    fn get(&self, key: String) -> KvsResult<String>;
    fn remove(&mut self, key: String) -> KvsResult<()>;
}
