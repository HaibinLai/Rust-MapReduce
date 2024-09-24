 use std::collections::HashMap;
 use std::sync::{Arc, Mutex};
 use rayon::prelude::*;
 use crate::{mapper, reducer};

 pub fn map_reduce_string(input: Vec<String>) -> HashMap<String, usize> {

     /*
     # Map
     */
     let results = Arc::new(Mutex::new(vec![]));

     /*
     rayon::vec impl<T: Send> IntoParallelIterator for Vec<T>
     fn into_par_iter(self) -> Self::Iter

     Converts self into a parallel iterator.
     */
     // 使用 Rayon 的并行处理
     input.into_par_iter().for_each(|line| {
         let mapped = mapper::map(line.as_str());
         results.lock().unwrap().push(mapped);
     });

     println!("Finish Map");

     // for handle in handles {
     //     handle.join().unwrap();
     // }

     println!("Finish Shuffling");

     /*
     # Reduce
     */
     let final_result = reducer::fetch_reduce(results);
     println!("Finish Reduce");

     final_result
 }
    // }



