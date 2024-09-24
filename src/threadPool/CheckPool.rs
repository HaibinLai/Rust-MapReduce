use std::thread::{self, JoinHandle};
use std::sync::{Arc, mpsc, Mutex};


type Job = Box<dyn FnOnce() + 'static + Send>;
enum Message {
    ByeBye,
    NewJob(Job),
}

struct Worker where
{
    _id: usize,
    t: Option<JoinHandle<()>>,
}

impl Worker
{
    fn new(id: usize, receiver: Arc::<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let t = thread::spawn( move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("do job from worker[{}]", id);
                        job();
                    },
                    Message::ByeBye => {
                        println!("ByeBye from worker[{}]", id);
                        break
                    },
                }
            }
        });

        Worker {
            _id: id,
            t: Some(t),
        }
    }
}

pub struct Pool {
    workers: Vec<Worker>,
    max_workers: usize,
    sender: mpsc::Sender<Message>
}

impl Pool where {
    pub fn new(max_workers: usize) -> Pool {
        if max_workers == 0 {
            panic!("max_workers must be greater than zero!")
        }
        let (tx, rx) = mpsc::channel();

        let mut workers = Vec::with_capacity(max_workers);
        let receiver = Arc::new(Mutex::new(rx));
        for i in 0..max_workers {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        Pool { workers: workers, max_workers: max_workers, sender: tx }
    }

    pub fn execute<F>(&self, f:F) where F: FnOnce() + 'static + Send
    {

        let job = Message::NewJob(Box::new(f));
        self.sender.send(job).unwrap();
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        for _ in 0..self.max_workers {
            self.sender.send(Message::ByeBye).unwrap();
        }
        for w in self.workers.iter_mut() {
            if let Some(t) = w.t.take() {
                t.join().unwrap();
            }
        }
    }
}

