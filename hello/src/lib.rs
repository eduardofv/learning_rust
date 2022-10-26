use std::{
    sync::{mpsc, Arc, Mutex}, 
    thread,
};

pub struct Worker {
    id: u32,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(
            id: u32, 
            receiver: Arc<Mutex<mpsc::Receiver<Job>>>
        ) -> Worker {
            let thread = thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                eprintln!("Worker {id} got a job, executing");
                job();
            });

            Worker { id, thread }
    }
}

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(num: usize) -> ThreadPool {
        assert!(num > 0);

        let mut threads = Vec::with_capacity(num);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..num {
            threads.push(Worker::new(id as u32, Arc::clone(&receiver)));
        }

        ThreadPool { threads, sender }
    }

    pub fn execute<F>(&self, handler: F) 
    where
        F: FnOnce() -> () + Send + 'static,
    {
        let job = Box::new(handler);

        self.sender.send(job).unwrap();
    }
}

