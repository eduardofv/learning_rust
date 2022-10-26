use std::thread;

pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(num: usize) -> ThreadPool {
        assert!(num > 0);

        let mut threads = Vec::with_capacity(num);

        for _ in 0..num {

        }

        ThreadPool { threads }
    }

    pub fn execute<F>(&self, handler: F) 
    where
        F: FnOnce() -> () + Send + 'static,
    {

    }
}

