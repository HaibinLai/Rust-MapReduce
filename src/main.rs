mod mapper;
mod reducer;
// mod ThreadPool;
mod test;
mod threadPool_test;
mod ThreadPool;
mod Framework;
mod mapreduce;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::fs;
use std::io::{self, BufRead};

// Map function


// Reduce function


fn main() -> Result<(), Box<dyn std::error::Error>> {


    let file_paths = vec!["books/Beowulf.txt", "books/Adventures_in_Wonderland.txt", "books/Pride_and_Prejudice.txt",
        "books/Sherlock_Holmes.txt", "books/The_Prince.txt", "books/Dorian_Gray.txt", "books/Dracula.txt",
                                "books/Dubliners.txt", "books/Great_Expectations.txt", "books/Siddhartha.txt"]; // 替换为你的文件路径
    let mut input = Vec::new();

    for path in file_paths {
        let file = fs::File::open(path)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            input.push(line);
        }
    }

    println!("Read complete");

   let final_result = Framework::map_reduce(input);

    // Print the result
    for (word, count) in final_result {
        if word == "CHAPTER" {
            println!("{}: {}", word, count);
        }
    }
    Ok(())
}


#[test]
fn test_framework(){
    let input = vec![
        String::from("hello world"),
        String::from("hello rust"),
        String::from("hello map reduce"),
        String::from("rust is great"),
    ];

    let final_result = Framework::map_reduce(input);
    // Print the result
    for (word, count) in final_result {
        println!("{}: {}", word, count);
    }
}

#[test]
fn test_main(){
    let input = vec![
        "hello world",
        "hello rust",
        "hello map reduce",
        "rust is great",
    ];


    let mut handles = vec![];
    let results = Arc::new(Mutex::new(vec![]));

    for line2 in input {
        let results_clone = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let mapped = mapper::map(line2);
            results_clone.lock().unwrap().push(mapped);
        });
        handles.push(handle);
    }

    println!("Finish Map");

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Finish Shuffling");

    let mut final_result = HashMap::new();
    for mapped_data in results.lock().unwrap().iter() {
        let reduced = reducer::reduce(mapped_data.clone());
        for (word, count) in reduced {
            *final_result.entry(word).or_insert(0) += count;
        }
    }

    println!("Finish Reduce");

    // Print the result
    for (word, count) in final_result {
        println!("{}: {}", word, count);
    }

}
