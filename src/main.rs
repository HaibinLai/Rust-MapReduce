mod framework;
mod mapper;
mod reducer;
// mod mapreduce;
// mod Model;
// mod thread_pool;
// mod threadPool_test;

use std::fs;
use std::io::{self, BufRead};



fn main() -> Result<(), Box<dyn std::error::Error>> {


    let file_paths =
        vec![
        "books/Beowulf.txt",
        "books/Adventures_in_Wonderland.txt",
        "books/Pride_and_Prejudice.txt",
        "books/Sherlock_Holmes.txt",
        "books/The_Prince.txt",
        "books/Dorian_Gray.txt",
        "books/Dracula.txt",
        "books/Dubliners.txt",
        "books/Great_Expectations.txt",
        "books/Siddhartha.txt"
        ]; // 替换为你的文件路径

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

    // use crate::framework::MapReduceString;
    let final_result = framework::map_reduce_string(input);

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

    // use framework;
    // use crate::framework::MapReduceString;
    let final_result = framework::map_reduce_string(input);
    // Print the result
    for (word, count) in final_result {
        println!("{}: {}", word, count);
    }
}

#[test]
fn test_main(){
    let input:Vec<String> = vec![
        "hello world".to_string(),
        "hello rust".to_string(),
        "hello map reduce".into(),
        "rust is great".to_string(),
    ];

    // use crate::framework::MapReduce_str;
    let final_result = framework::map_reduce_string(input);

    // Print the result
    for (word, count) in final_result {
        println!("{}: {}", word, count);
    }

}
