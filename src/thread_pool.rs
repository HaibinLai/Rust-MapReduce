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

/**
# Message

There are 3 types of message:

1. JobEnd: to end a job

2. New(Job)

3. JobErr
*/
enum Message {
    JobEnd,
    NewJob(Job),
    JobErr,
}

/**
# ThreadPool

@workers: the activate workers in pool

@max_workers: the thread num we have in pool

@sender:The single-consumer used for sending tasks to worker threads.

## Thread Model:

The sender in the thread pool implementation is part of an *MPSC* (multi-producer, single-consumer)
channel used for sending tasks to worker threads. Here's a detailed explanation of its role:

**Channel Creation**: mpsc::channel() creates a new channel consisting of a sender (sender) and a receiver (receiver).
The sender allows you to send messages (in this case, jobs) to the receiver.

**Submitting Tasks**: When you call the execute method, the task is wrapped in a Box and sent to the channel using self.sender.send(job).
This is how you submit work to the thread pool.

**Worker Threads**: Each worker thread listens for tasks by calling receiver.recv().
When a task is received, the thread executes it.

**Concurrency**: This design allows multiple threads to submit tasks concurrently (multiple producers)
while the worker threads consume and execute tasks one at a time (single consumer).

*/
pub struct Pool {
    workers: Vec<Worker>,
    max_workers: usize,
    sender: mpsc::Sender<Message>
}

/**
# Worker

@worker_id: id for worker, let it be the same as thread id

@t: An owned permission to join on a thread (block on its termination).
A JoinHandle detaches the associated thread when it is dropped,
which means that there is no longer any handle to the thread and no way to join on it.
*/
struct Worker where
{
    worker_id: usize,
    t: Option<JoinHandle<()>>,
}

impl Worker
{
    fn new(id: usize, receiver: Arc::<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        todo!()
    }
}



impl Pool where {
    pub fn new(max_workers: usize) -> Pool {
        todo!()
    }

    pub fn execute<F>(&self, f:F) where F: FnOnce() + 'static + Send
    {
        todo!()
    }
}

impl Drop for Pool {
    fn drop(&mut self) {
        todo!()
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
        p.execute(|| println!("do new job5"));
    }
}
