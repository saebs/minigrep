use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    // old code below
    // let queary = &args[1]; 
    // let filename = &args[2];
    // old code
    // let (query, filename) = parse_config(&args);

    let config = parse_config(&args);

// old code
    // println!("Searching  for {}", query);
    // println!("In file {}", filename);

    println!("Searching  for {}", config.query);
    println!("In file {}", config.filename);

    // Reading a file
    let mut f = File::open(config.filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Sometihng went wrong reading the file");
    println!("With text:\n{}", contents);

    // Refactoring to Improve Modularity and Error Handling.
    
}

struct Config {
    query: String,
    filename: String,
}

//  fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let filename = &args[2];
//     (query, filename)
//  }

 fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config { query, filename }
 }
