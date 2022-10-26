use std::thread;

pub struct Worker {
    id: u32,
    handle: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: u32) -> Worker {
        Worker {
            id: id,
            handle: thread::spawn(|| {}),
        }
    }
}

pub struct ThreadPool {
    threads: Vec<Worker>,
}

impl ThreadPool {
    pub fn new(num: usize) -> ThreadPool {
        assert!(num > 0);

        let mut threads = Vec::with_capacity(num);

        for id in 0..num {
            threads.push(Worker::new(id as u32));
        }

        ThreadPool { threads }
    }

    pub fn execute<F>(&self, handler: F) 
    where
        F: FnOnce() -> () + Send + 'static,
    {

    }
}

