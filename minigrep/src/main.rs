mod config; // basically tells Rust to look for a file named config.rs in the same directory as main.rs and include its contents as a module named config
mod reader; // same as above, but for reader.rs

use crate::config::Config;
use crate::reader::Reader;
use minigrep::{search_case_insensitive, search_with_closure};

use std::env;
use std::error;
use std::process;

fn main() {
    // call the run function and handle any errors that occur during execution
    // allows exits gracefully and instead of a lot of Rust error handling, we do it our own way
    if let Err(err) = run() {
        eprintln!("Problem: {}", err); // eprintln! is like println!, but prints to stderr instead of stdout
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn error::Error>> {
    // create a new Config struct from the command line arguments
    // and handle any errors that occur during parsing with the ? operator
    let cfg = Config::new_from_iter(env::args())?;

    // create a new Reader struct from the file path in the Config struct
    // and handle any errors that occur during reading with the ? operator
    let reader = Reader::new(&cfg.file_path);
    let contents = reader.read()?;

    let results = if cfg.ignore_case {
        search_case_insensitive(&cfg.query, &contents)
    } else {
        search_with_closure(&cfg.query, &contents)
    };

    if results.is_empty() {
        println!("No matches found");
    } else {
        for line in results {
            println!("{}", line);
        }
    }

    Ok(())
}
