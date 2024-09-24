// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};

// use std::thread;
// use crate::{mapper};

// pub fn fetch_reduce(input: Vec<String>) {
//
//     for line in input {
//         let results_clone = Arc::clone(&results);
//         let handle = thread::spawn(move || {
//             let mapped = map(line.as_str());
//             results_clone.lock().unwrap().push(mapped);
//         });
//         handles.push(handle);
//     }
//
// }

pub fn map(data: &str) -> Vec<(String, usize)> {
    data.split_whitespace()
        .map(|word| (word.to_string(), 1))
        .collect()
}