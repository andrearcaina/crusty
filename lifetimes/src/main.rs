fn main() {
    let r;

    {
        let x = 5;
        r = &x;

        println!("r: {}", r);
    }

    // this will cause an error because x goes out of scope and r is a reference to x
    // r is a reference to x, and x is dropped at the end of the inner scope, so r is a dangling reference.
    // println!("r: {}", r);

    // the following will work because r is a reference to x, and x is still in scope when we print r
    let x = 5;
    let r = &x;
    println!("r: {}", r);
    println!("x: {}", x);
    // here, x has a lifetime that is at least as long as r, so r is valid when we print it
    // both r and x is dropped at the end of the main function

    // generic lifetimes in functions
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // the static lifetime
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);
    // this means that the string literal "I have a static lifetime." is stored in the binary of the program
    // and is available for the entire duration of the program, so it has a static lifetime
    // in this case, it will just print "I have a static lifetime." and then the program will end
    // but if we had a longer running program, we could still use s and it would still be valid

    let result =
        longest_with_an_announcement(string1.as_str(), string2, "This is an announcement!");

    println!("The longest string is {}", result);
}

// this function takes two string slices and returns the longest one
// but it won't work because the lifetimes of the string slices are not specified, so the compiler doesn't know how long the returned reference will be valid for
// what this means is that the function longest has no idea how long x and y will be valid for, so it can't guarantee that the reference
// it returns will be valid for any particular lifetime
// rust needs to specify the lifetimes of the references in the function signature to ensure that the returned reference is valid for the same lifetime as the input references
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }

// what this does is it tells the compiler that the function longest has a generic lifetime parameter 'a,
// and that the references x and y must have the same lifetime 'a, and that the returned reference will also have the same lifetime 'a
// a is just a variable name for the lifetime, and it can be any name, but it is convention to use 'a, 'b, etc. for lifetime parameters
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// each function parameter can have their own lifetime, and the return type can also have its own lifetime
// for example, x can have a lifetime 'a, y can have a lifetime 'b, and the return type can have a lifetime 'c
// fn some_function<'a, 'b, 'c>(x: &'a str, y: &'b str) -> &'c str {
// some code that uses x and y and returns a reference with lifetime 'c
// this is just an example, and it won't compile because we don't have any code that returns a reference with lifetime 'c
// but it shows that we can have multiple lifetime parameters
//     x
// }

use std::fmt::Display;

// we can also have generic type parameters in addition to lifetime parameters
// in this example, we have a generic type parameter T that must implement the Display trait, and we can use it in the function body to print the announcement
// the lifetime parameter 'a is still used to specify the lifetimes of the string slices, and the generic type parameter T is used to specify the type of the announcement
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}
