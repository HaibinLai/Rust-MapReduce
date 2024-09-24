use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn fetch_reduce(results: Arc<Mutex<Vec<Vec<(String,usize)>>>>) -> HashMap<String, usize> {
    use std::collections::HashMap;
    let mut final_result = HashMap::new();
    for mapped_data in results.lock().unwrap().iter() {
        let reduced = reduce(mapped_data.clone());
        for (word, count) in reduced {
            *final_result.entry(word).or_insert(0) += count;
        }
    }
    final_result
}

pub fn reduce(mapped_data: Vec<(String, usize)>) -> HashMap<String, usize> {
    let mut result = HashMap::new();
    for (word, count) in mapped_data {
        *result.entry(word).or_insert(0) += count;
    }
    result
}
