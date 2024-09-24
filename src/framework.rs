 use std::collections::HashMap;
 use std::sync::{Arc, Mutex};
 use std::thread;
 use crate::{mapper, reducer};

 pub fn map_reduce_string(input: Vec<String>) -> HashMap<String, usize> {

     /*
     # Map
     */
     let mut handles = vec![];
     let results = Arc::new(Mutex::new(vec![]));

     for line in input {
         let results_clone = Arc::clone(&results);
         let handle = thread::spawn(move || {
             let mapped = mapper::map(line.as_str());
             results_clone.lock().unwrap().push(mapped);
         });
         handles.push(handle);
     }

     println!("Finish Map");

     for handle in handles {
         handle.join().unwrap();
     }

     println!("Finish Shuffling");

     /*
     # Reduce
     */
     let final_result = reducer::fetch_reduce(results);
     println!("Finish Reduce");

     final_result
 }
    // }



