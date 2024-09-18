mod mapper;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

// Map function


// Reduce function
fn reduce(mapped_data: Vec<(String, usize)>) -> HashMap<String, usize> {
    let mut result = HashMap::new();
    for (word, count) in mapped_data {
        *result.entry(word).or_insert(0) += count;
    }
    result
}

fn main() {
    let input = vec![
        "hello world",
        "hello rust",
        "hello map reduce",
        "rust is great",
    ];

    let mut handles = vec![];
    let results = Arc::new(Mutex::new(vec![]));

    for line in input {
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

    let mut final_result = HashMap::new();
    for mapped_data in results.lock().unwrap().iter() {
        let reduced = reduce(mapped_data.clone());
        for (word, count) in reduced {
            *final_result.entry(word).or_insert(0) += count;
        }
    }

    // Print the result
    for (word, count) in final_result {
        println!("{}: {}", word, count);
    }
}
