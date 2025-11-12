use std:: {
    sync::{
        mpsc,
        Arc,
        Mutex
    },
    thread,
    time::Duration,
};

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            let receiver = Arc::clone(&receiver);
            let thread = thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv();
                match job {
                    Ok(job) => {
                        println!("Worker {id} is running...");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} is terminated");
                        break;
                    }
                }
            });

            workers.push(Worker { id, thread: Some(thread) });
        }
        ThreadPool { workers, sender }
    }

    fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("ThreadPool is terminating...");
        drop(&self.sender);
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
        println!("Every Workers are terminated");
    }
}

fn main() {
    let pool = ThreadPool::new(3);
    println!("Thread pool is allocated");

    for i in 1..=5 {
        pool.execute(move || {
            println!("Task {i} is started!");
            thread::sleep(Duration::from_secs(1));
            println!("Task {i} is completed!");
        });
    }

    println!("Main Thread is waiting...");
    thread::sleep(Duration::from_secs(6));
}
