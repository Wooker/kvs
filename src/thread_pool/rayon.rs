use crate::{thread_pool::ThreadPool, KvsResult};
use rayon::ThreadPoolBuilder;

pub struct RayonThreadPool(rayon::ThreadPool);

impl ThreadPool for RayonThreadPool {
    fn new(threads: u32) -> KvsResult<Self>
    where
        Self: Sized,
    {
        let pool = Self(
            ThreadPoolBuilder::new()
                .num_threads(threads as usize)
                .build()
                .unwrap(),
        );

        Ok(pool)
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.0.spawn(job)
    }
}
