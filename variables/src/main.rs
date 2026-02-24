fn main() {
    // type inference
    let x = 9;
    println!("The value of x is: {}", x);
    // x = 6; x cannot be changed because it is immutable by default
    // in order to change x, we need to declare it as mutable
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 11;
    println!("The value of y is: {}", y);

    // can also explicitly declare the type
    let z: i32 = 12;
    println!("The value of z is: {}", z);

    // constants are always immutable and must be declared with a type
    // can be declared in any scope (including global scope)
    const PI: f64 = 3.14;
    println!("The value of PI is: {}", PI);

    // Shadowing: this does not mutate the existing variable
    // it creates a new variable with the same name, hiding the previous one
    // the previous x is not accessible in this scope or changed, just hidden
    let x = 5;
    println!("The value of x is: {}", x);

    let x = x + 1; // another shadowed x. in reality there's 3 variables called "x" at this point
    {
        let x = x * 2; // another shadowed x. in reality there's 4 variables called "x" at this point
        // this x is only accessible in this scope due to the curly braces surrounding the block
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x in the outer scope is: {}", x);

    let spaces = "   ";
    println!("The value of spaces is: \"{}\"", spaces);

    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    // u cannot do this though
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // this is because we're not allowed to mutate a variables size
    // mut allows mutation of value to the typed variable, not mutating the actual type of the variable
}
