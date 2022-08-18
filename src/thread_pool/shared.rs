use crate::{thread_pool::ThreadPool, KvsResult};
use std::{thread, sync::{mpsc::{channel, Sender, Receiver}, Arc, Mutex}};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct SharedQueueThreadPool {
    sender: Option<Sender<Job>>,
    workers: Vec<Worker>,
}

impl ThreadPool for SharedQueueThreadPool {
    fn new(threads: u32) -> KvsResult<Self>
    where Self: Sized {
        let mut workers: Vec::<Worker> = Vec::default();
        let (tx, rx) = channel::<Job>();

        let receiver = Arc::new(Mutex::new(rx));
        for i in 0..threads {
            workers.push(Worker::new(i.try_into().unwrap(), receiver.clone()));
        }

        Ok(Self {
            sender: Some(tx),
            workers
        })
    }

    fn spawn<F>(&self, job: F)
    where F: FnOnce() + Send + 'static {
        let job = Box::new(job);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for SharedQueueThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                println!("Stopping thread");
                thread.join().unwrap();
            }
        }
    }
}


struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
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

        Self { id, thread: Some(thread) }
    }
}
