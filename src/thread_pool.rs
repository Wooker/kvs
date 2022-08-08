use crate::KvsResult;

pub trait ThreadPool {
    fn new(threads: u32) -> KvsResult<Self> where Self: Sized;
    fn spawn<F>(&self, job: F) where F: FnOnce() + Send + 'static;
}

pub struct NaiveThreadPool {
}

impl ThreadPool for NaiveThreadPool {
    fn new(_: u32) -> KvsResult<Self>
    where Self: Sized {
        Ok(Self {})
    }

    fn spawn<F>(&self, job: F)
    where F: FnOnce() + Send + 'static {
        job();
    }
}

pub struct SharedQueueThreadPool {
}

impl ThreadPool for SharedQueueThreadPool {
    fn new(threads: u32) -> KvsResult<Self>
    where Self: Sized {
        Ok(Self {})
    }

    fn spawn<F>(&self, job: F)
    where F: FnOnce() + Send + 'static { }
}

pub struct RayonThreadPool {
}

impl ThreadPool for RayonThreadPool {
    fn new(threads: u32) -> KvsResult<Self>
    where Self: Sized {
        Ok(Self {})
    }

    fn spawn<F>(&self, job: F)
    where F: FnOnce() + Send + 'static { }
}
