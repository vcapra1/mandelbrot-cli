use std::{
    thread,
    sync::{mpsc, Arc, Mutex}
};

enum Message {
    NewJob(Job),
    Terminate
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    job_sender: mpsc::Sender<Message>
}

trait FnBox {
    fn call_box(self: Box<Self>, stop_sender: mpsc::Sender<()>);
}

impl<F: FnOnce(mpsc::Sender<()>)> FnBox for F {
    fn call_box(self: Box<F>, stop_sender: mpsc::Sender<()>) {
        (*self)(stop_sender)
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize, stop_sender: mpsc::Sender<()>) -> ThreadPool {
        // Make sure size isn't zero
        assert!(size > 0);

        // Get a channel for sending Jobs to the workers
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        // Create a vector of the workers
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&rx), stop_sender.clone()));
        }

        ThreadPool { workers, job_sender: tx }
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce(mpsc::Sender<()>) + Send + 'static {
        let job = Box::new(f);

        self.job_sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Send Terminate message to all threads
        for _ in &mut self.workers {
            println!("Terminating...");
            self.job_sender.send(Message::Terminate).unwrap();
        }
        
        // Wait for threads to shut down
        for worker in &mut self.workers {
            println!("Waiting...");
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    _id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>, stop_sender: mpsc::Sender<()>) -> Worker {
        // Create the thread
        let thread = thread::spawn(move || {
            // Loop until terminate message is received
            loop {
                // Get the message
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        // Run the job
                        job.call_box(stop_sender.clone());
                    },
                    Message::Terminate => {
                        // Break out of the loop
                        break
                    }
                }
            }
        });

        // Return a worker with id and join handle to thread
        Worker { _id: id, thread: Some(thread) }
    }
}
