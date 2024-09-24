use std::collections::HashMap;

pub fn reduce(mapped_data: Vec<(String, usize)>) -> HashMap<String, usize> {
    let mut result = HashMap::new();
    for (word, count) in mapped_data {
        *result.entry(word).or_insert(0) += count;
    }
    result
}