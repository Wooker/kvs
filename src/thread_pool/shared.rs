use crate::{thread_pool::ThreadPool, KvsResult};
use std::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct SharedQueueThreadPool {
    sender: Option<Sender<Job>>,
    workers: Vec<Worker>,
}

impl ThreadPool for SharedQueueThreadPool {
    fn new(threads: u32) -> KvsResult<Self>
    where
        Self: Sized,
    {
        let mut workers: Vec<Worker> = Vec::default();
        let (tx, rx) = channel::<Job>();

        let receiver = Arc::new(Mutex::new(rx));
        for i in 0..threads as usize {
            println!("New thread");
            workers.push(Worker::new(i, receiver.clone()));
        }

        Ok(Self {
            sender: Some(tx),
            workers,
        })
    }

    fn spawn<F>(&self, job: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(job);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for SharedQueueThreadPool {
    fn drop(&mut self) {
        // dbg!(&self.workers);
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                println!("Stopping thread");
                if !thread.is_finished() {
                    println!("Not finished");
                }
            }
        }
    }
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let count: i32 = 0;
        let thread = thread::spawn(move || loop {
            match receiver.lock().unwrap().recv() {
                Ok(job) => {
                    println!("Worker {} took the job.", id);
                    job();
                }
                Err(_) => {
                    println!("Worker faced an error. Bye!");
                    break;
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}
