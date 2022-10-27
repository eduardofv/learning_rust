use std::{
    sync::{mpsc, Arc, Mutex}, 
    thread,
};

pub struct Worker {
    id: u32,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(
            id: u32, 
            receiver: Arc<Mutex<mpsc::Receiver<Job>>>
        ) -> Worker {
            let thread = thread::spawn(move || loop {
                match receiver.lock().unwrap().recv() {
                    Ok(job) => {
                        println!("Worker {id} got a job, executing");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected, shutting down");
                        break;
                    }
                }
            });

            Worker { id, thread: Some(thread) }
    }
}

pub struct ThreadPool {
    threads: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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

        ThreadPool { threads, sender: Some(sender), }
    }

    pub fn execute<F>(&self, handler: F) 
    where
        F: FnOnce() -> () + Send + 'static,
    {
        let job = Box::new(handler);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.threads {
            println!("Shutting down worker {}", worker.id);

            if let Some(t) = worker.thread.take() {
                t.join().unwrap();
            }
        }
    }
}

