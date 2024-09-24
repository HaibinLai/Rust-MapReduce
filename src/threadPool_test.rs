use std::sync::{Arc, Mutex};
use std::thread;
use crate::test::Pool;

#[cfg(test)]
mod tests {
    use crate::mapper;
    use super::*;
    #[test]
    fn it_works() {
        let p = Pool::new(4);
        p.execute(|| println!("do new job1"));
        p.execute(|| println!("do new job2"));
        p.execute(|| println!("do new job3"));
        p.execute(|| println!("do new job4"));
        p.execute(|| println!("do new job5"));
        p.execute(|| println!("do new job6"));
    }

    #[test]
    fn it_map() {
        let p = Pool::new(4);
        let input = vec![
            "hello world",
            "hello rust",
            "hello map reduce",
            "rust is great",
        ];

        let mut handles = vec![];
        let results = Arc::new(Mutex::new(vec![]));

        for line in input {
            p.execute(||{ mapper::map(line);});
            let results_clone = Arc::clone(&results);
            let handle = thread::spawn(move || {
                let mapped = mapper::map(line);
                results_clone.lock().unwrap().push(mapped);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
