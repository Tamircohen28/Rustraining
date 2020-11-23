use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

/// empty struct to inidicate error had occured
pub struct PoolCreationError;

/// a job as defined in the system
type Job = Box<dyn FnOnce() + Send + 'static>;

/// represent messages between threads
enum Message {
    NewJob(Job),
    Terminate,
}

/// struct to describe working thread
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

/// pool of Threads
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver
            .lock().expect("Mutex Lock err had occured in Woker")
            .recv().expect("Recv err had occured in Woker");

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl ThreadPool {

    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will return err if size is 0.
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        // check if size is invalid
        match size {
            0 => return Err(PoolCreationError),
            _ => ()
        };

        // creat new channel, main thread has source and all threads listen to it
        let (sender, receiver) = mpsc::channel();

        // set receiver to be Arc so it can be shared and mutex so no race will occur
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }

    /// send job to thread
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

/// Dropping all threads in a elegent way :)
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        // send terminate to all workers so all threads will reach end of function
        for _ in &self.workers {
            self.sender.send(Message::Terminate).expect("sending Terminate to workers had falied");
        }

        println!("Shutting down all workers.");

        // after all thread had ended their job we want to close them using join()
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            
            // using take() to take thread from Option and leave None there, then kill thread
            if let Some(thread) = worker.thread.take() {
                thread.join().expect("thread killing had falied");
            }
        }
    }
}