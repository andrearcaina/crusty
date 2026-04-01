mod config; // basically tells Rust to look for a file named config.rs in the same directory as main.rs and include its contents as a module named config
mod reader; // same as above, but for reader.rs

use crate::config::Config;
use crate::reader::Reader;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // build a Config struct from the command line arguments, and handle any errors that occur during parsing
    // unwrap_or_else will call the provided closure if the Result is an Err, allowing us to print an error message and exit the program gracefully
    let cfg = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1); // same as os.Exit(1) in Go, it terminates the program with a non-zero exit code to indicate an error
    });

    let reader = Reader::new(&cfg.file_path); // create a new Reader instance using the file path specified in the Config struct

    // read the contents of the file specified in the Config struct, and handle any errors that occur during file reading
    let contents = reader.read().unwrap_or_else(|err| {
        println!("Problem reading file: {}", err);
        process::exit(1); // same as os.Exit(1) in Go, it terminates the program with a non-zero exit code to indicate an error
    });

    println!("File contents:\n{}", contents); // print the contents of the file to the console
}
