// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};
// use std::thread;
//
// mod mapper {
//     pub fn map(line: &str) -> Vec<(String, usize)> {
//         // 示例的映射逻辑，返回单词及其计数
//         line.split_whitespace()
//             .map(|word| (word.to_string(), 1))
//             .collect()
//     }
// }
//
// mod reducer {
//     use std::collections::HashMap;
//
//     pub fn reduce(mapped_data: Vec<(String, usize)>) -> HashMap<String, usize> {
//         let mut result = HashMap::new();
//         for (word, count) in mapped_data {
//             *result.entry(word).or_insert(0) += count;
//         }
//         result
//     }
// }
//
// pub fn map_reduce<'a>(input: Vec<&'a str>) -> HashMap<String, usize> {
//     let mut handles = vec![];
//     let results = Arc::new(Mutex::new(vec![]));
//
//     for line in input {
//         let results_clone = Arc::clone(&results);
//         let handle = thread::spawn(move || {
//             let mapped = mapper::map(line);
//             results_clone.lock().unwrap().push(mapped);
//         });
//         handles.push(handle);
//     }
//
//     println!("Finish Map");
//
//     for handle in handles {
//         handle.join().unwrap();
//     }
//
//     println!("Finish Shuffling");
//
//     let mut final_result = HashMap::new();
//     for mapped_data in results.lock().unwrap().iter() {
//         let reduced = reducer::reduce(mapped_data.clone());
//         for (word, count) in reduced {
//             *final_result.entry(word).or_insert(0) += count;
//         }
//     }
//
//     println!("Finish Reduce");
//
//     final_result
// }
//
// #[test]
// fn main_test() {
//     let input = vec![
//         "hello world",
//         "hello rust",
//         "hello map reduce",
//         "rust is great",
//     ];
//
//     let result = map_reduce(input);
//     for (word, count) in result {
//         println!("{}: {}", word, count);
//     }
// }
