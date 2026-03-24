fn main() {
    // panic!("crash and burn");
    // this will just panic and exit the program
    //
    // let v = vec![1, 2, 3];
    //
    // v[100]; this will panic because the index is out of bounds

    // we can use the Result type to handle panics without panicking
    let v = vec![1, 2, 3];
    let result = v.get(100);
    match result {
        Some(value) => println!("value: {}", value),
        None => println!("index out of bounds"),
    }

    // can use Err() to handle panics without panicking

    use std::fs::File;
    use std::io::ErrorKind;

    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        // if file exists, print a message and return the file (greeting_file will now be the file object)
        Ok(file) => {
            println!("hello.txt file exists");
            file
        }
        // if file does not exist, create it and return the file object
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    println!("created file hello.txt");
                    fc
                }
                // panic if cannot create the file
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _ => {
                // panic if cannot open the file
                panic!("Problem opening the file: {:?}", error);
            }
        },
    };

    // a cleaner version is using the unwrap_or_else method with a closure
    // although this version doesn't handle if the file already exists
    let _greeting_file = File::open("sup.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            println!("sup.txt file does not exist, creating it");
            File::create("sup.txt").unwrap_or_else(|e| panic!("Problem creating the file: {:?}", e))
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // can use unwrap to return the file, panicking if the file does not exist
    let _greeting_file = File::open("sup.txt").unwrap();

    // can also use the .expect method to return the file, panicking if the file does not exist
    // let _greeting_file = File::open("what.txt").expect("what.txt file does not exist");
}

// so many ways to handle errors in Rust
use std::fs::File;
use std::io::{self, Read};

fn _read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }

    // can also use the ? operator to return the file, panicking if the file does not exist
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?; this is equivalent to the match expression above
    //
    // can also use the ? for file opening
    // let mut username = String::new();
    // File::open("hello.txt")?.read_to_string(&mut username)?; this is equivalent to the match expression above
    //
    // can even just use fs::read_to_string (simple one liner)
    // use std::fs;
    // use std::io;
    // fs::read_to_string("hello.txt")
    //
    // the ? operator can only be used in functions that return a Result
    // for example we cannot do let greeting_file = File::open("hello.txt")?; this will not compile because File::open returns a File, not a Result
    // unless we wrap it in a Result using Ok(File::open("hello.txt")?) (check test function below)
}

fn _test() -> Result<(), Box<dyn std::error::Error>> {
    let _greeting_file = File::open("hello.txt")?;
    Ok(())
}
