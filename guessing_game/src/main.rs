use std::io::{self, Write}; // import certain modules from std::io (self is itself "io", and Write is a module)

fn main() {
    const TARGET: i8 = 3;

    print!("Guess the number: ");
    io::stdout().flush().unwrap(); // have to do this for the print statement since print! doesn't flush the buffer
    // print! vs println!: println! flushes the buffer automatically with newline

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Please enter a valid number");

    // kinda weird of an error catch but idk maybe i gotta get used to it
    if let Ok(guess) = guess.trim().parse::<i8>() {
        if guess == TARGET {
            println!("You got the number correct: {}!", TARGET);
        } else {
            println!("You guessed the wrong number");
        }
    } else {
        println!("Invalid input");
    }
}
