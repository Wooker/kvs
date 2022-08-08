use crate::KvsResult;

pub mod kvstore;
pub mod sled;

pub trait KvsEngine
where Self: Clone + Send + 'static {
    fn set(&mut self, key: String, val: String) -> KvsResult<()>;
    fn get(&self, key: String) -> KvsResult<String>;
    fn remove(&mut self, key: String) -> KvsResult<()>;
}
