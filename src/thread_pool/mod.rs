use crate::KvsResult;

pub mod naive;
pub mod rayon;
pub mod shared;

pub trait ThreadPool {
    fn new(threads: u32) -> KvsResult<Self>
    where
        Self: Sized;
    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static;
}
