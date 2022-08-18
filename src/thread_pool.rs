use std::{thread, sync::mpsc::channel};
use rayon::ThreadPoolBuilder;
use crate::KvsResult;

pub trait ThreadPool {
    fn new(threads: u32) -> KvsResult<Self> where Self: Sized;
    fn spawn<F>(&self, job: F) where F: FnOnce() + Send + 'static;
}

pub struct NaiveThreadPool;

impl ThreadPool for NaiveThreadPool {
    fn new(_: u32) -> KvsResult<Self>
    where Self: Sized {
        Ok(Self)
    }

    fn spawn<F>(&self, job: F)
    where F: FnOnce() + Send + 'static {
        thread::spawn(job);
    }
}

pub struct SharedQueueThreadPool { }

impl ThreadPool for SharedQueueThreadPool {
    fn new(_threads: u32) -> KvsResult<Self>
    where Self: Sized {
        let (_tx, _rx) = channel::<()>();
        /*
        for _ in 0..threads {
            let rx = JobReceiver(rx.clone());
            thread::Builder::new().spawn(move || run_jobs(rx));
        }
        */

        Ok(Self {})
    }

    fn spawn<F>(&self, job: F)
    where F: FnOnce() + Send + 'static {
        thread::spawn(job);
    }
}

pub struct RayonThreadPool(rayon::ThreadPool);

impl ThreadPool for RayonThreadPool {
    fn new(threads: u32) -> KvsResult<Self>
    where Self: Sized {
        Ok(Self(ThreadPoolBuilder::new().num_threads(threads as usize).build().unwrap()))
    }

    fn spawn<F>(&self, job: F)
    where F: FnOnce() + Send + 'static {
        self.0.spawn(job)
    }
}
