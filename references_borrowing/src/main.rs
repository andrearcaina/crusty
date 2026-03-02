use std::string::String;

fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1); // & represents a reference, which allows us to refer to some value without taking ownership of it

    // what this means is we can still use s1 after we calculate its length, because we only borrowed it, not took ownership of it

    println!("The length of '{}' is {}.", s1, len);

    s1.push_str(" world");

    println!("{}!", s1);

    // this doesnt work below
    // change(&s1);
    // because we are trying to change s1, but we only have a reference to it, not ownership of it. We need to pass a mutable reference to s1 in order to change it
    // we can do this by using &mut s1 instead of &s1, and we also need to change the function signature to accept a mutable reference

    change(&mut s1);

    println!("{}!", s1);

    let mut s2 = String::from("Sup");

    let r1 = &mut s2;
    // let r2 = &mut s2; // this doesnt work because we can only have one mutable reference to a particular piece of data in a particular scope. This is to prevent data races

    println!("{}", r1);

    // now we can modify both s2 and r1. if we modify r1, we are also modifying s2, and vice versa

    change(r1);
    println!("{}", s2);

    change(&mut s2); // equivalent of just doing change (r1) because r1 is a mutable reference to s2, so we can use it to modify s2
    println!("{}", s2);

    let mut s2 = String::from("Sup");

    {
        let r1 = &mut s2; // this is fine because r1 goes out of scope at the end of this block, so we can create a new mutable reference to s2 in the next block
        println!("r1 in scope before change {}", r1);

        change(r1);
        println!("r1 in scope after change {}", s2);
    }

    let r2 = &mut s2; // this is fine because r1 is out of scope, so we can create a new mutable reference to s2
    println!("r2 in outer scope with change (due to r1 scope) {}", r2);

    // we cannot have a mutable and an immutable reference to the same data at the same time
    // let r1 = &s2; // no problem
    // let r2 = &s2; // no problem (can have multiple immutable references)
    // let r3 = &mut s2; // this will fail
    // but, this works:

    let mut s2 = String::from("Sup");

    let r1 = &s2; // no problem
    let r2 = &s2; // no problem

    println!("{}, {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s2; // no problem
    println!("{}", r3);

    change(r3);

    println!("{}", r3);

    let reference = dangle();

    println!("reference to nothing: {}", reference);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(" world");
}

// make sure this function does not return a reference to a value that will go out of scope
// this will compile, but it will not work because we are returning a reference to a value that will go out of scope when the function ends
/*
 * fn dangle() -> &String {
 *     let s = String::from("hello");
 *
 *     &s
 * }
 */
// in order to fix this, we can return the String itself, instead of a reference to it.
// This way, the ownership of the String will be transferred to the caller, and we won't have a dangling reference.
fn dangle() -> String {
    let s = String::from("hello");

    s
}
