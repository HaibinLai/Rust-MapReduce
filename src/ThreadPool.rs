// implement by Haibin Lai
/**
# thread

The threading model

An executing Rust program consists of a collection of native OS threads,each with their own stack and local state.
Threads can be named, and provide some built-in support for low-level synchronization.
*/
use std::thread::{self, JoinHandle};

/**
# sync

Useful synchronization primitives.
The need for synchronization

Conceptually, a Rust program is a series of operations which will be executed on a computer.
The timeline of events happening in the program is consistent with the order of the operations in the code.
*/
use std::sync::{Arc, mpsc, Mutex};

/**
# FnOnce()

Instances of FnOnce can be called, but might not be callable multiple times.
Because of this, if the only thing known about a type is that it implements FnOnce, it can only be called once.

---

# Send

pub unsafe trait Send
Types that can be transferred across thread boundaries.
This trait is automatically implemented when the compiler determines it's appropriate.
*/
type Job = Box<dyn FnOnce() + 'static + Send>;


enum Message {
    JobFinish,
    NewJob(Job),
}

pub struct Pool {
    workers: Vec<Worker>,
    max_workers: usize,
    sender: mpsc::Sender<Message>
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
                    Message::JobFinish => {
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
            self.sender.send(Message::JobFinish).unwrap();
        }
        for w in self.workers.iter_mut() {
            if let Some(t) = w.t.take() {
                t.join().unwrap();
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let p = Pool::new(4);
        p.execute(|| println!("do new job1"));
        p.execute(|| println!("do new job2"));
        p.execute(|| println!("do new job3"));
        p.execute(|| println!("do new job4"));
    }
}
