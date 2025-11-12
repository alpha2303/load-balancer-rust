use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

use std::fmt;

#[derive(Debug, Clone)]
pub enum ThreadPoolError {
    CreationError(String),
}

impl fmt::Display for ThreadPoolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ThreadPoolError::CreationError(message) => {
                write!(f, "ThreadPoolError: {}", message)
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        return Self {
            id,
            thread: thread::spawn(move || {
                loop {
                    println!("Worker {} is waiting for job...", id);
                    let job = receiver.lock().unwrap().recv().unwrap();
                    println!("Worker {} is executing job...", id);
                    job();
                }
            }),
        };
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<Self, ThreadPoolError> {
        if size == 0 {
            return Err(ThreadPoolError::CreationError(
                "Thread Pool size is 0.".to_string(),
            ));
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }
        return Ok(Self { workers, sender });
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job: Job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
