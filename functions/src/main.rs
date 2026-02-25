fn main() {
    println!("Hello, world!");

    another_function();

    parameter_function(5, 'Z');

    // statements and expressions
    // different from anonymous functions, they are just block of code that can be used as an expression, and they can return a value
    // the most similar is C, but Rust is different and actually "returns" the last expression in the block, without the need of a return keyword,
    // and it can be used in any place where an expression is expected
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    println!("called five(): {}", five());
    println!("called six(): {}", six());
}

fn another_function() {
    println!("Another function.");
}

fn parameter_function(x: i32, ch: char) {
    println!("The value of x is: {}", x);
    println!("The value of ch is: {}", ch);
}

fn five() -> i32 {
    5 // no need for a return keyword, the last expression is returned already. u can still do return 5; if you want to
}

fn six() -> i32 {
    return 6; // this is also valid, but not Rust idiomatic
}
