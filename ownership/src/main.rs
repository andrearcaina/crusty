fn main() {
    // each value in Rust has an owner
    // there can only be one owner at a time (for each value)
    // when the owner goes out of scope, the value will be dropped

    {
        // s is not valid here, since it's not yet declared
        let s = "hello"; // s is valid from this point forward

        println!("{} world!", s);
    } // this scope is now over, and s is no longer valid

    // println!("{} world!", s); // s is not valid here, so we can't use it

    // to illustrate rules of ownership, we need a data type more complex than the data types already covered
    // String is different from string literals

    // use the String struct from the std string library (auto imported already, this is just for readability)
    use std::string::String;

    // example of string literals
    let s1 = "hello"; // this is a string literal, which is stored in the binary and is immutable (type &str)
    println!("{} world!", s1);

    // example of String type
    let mut s2 = String::from("hello"); // this is a String, which is stored on the heap and is mutable (type String)
    // from is an associated function of the String type (meaning that String structs have this function that "constructs" a String from a string literal)
    // associated functions are functions that are defined in the context of a struct, and are called using the syntax StructName::function_name (like String::from)
    // another example of an associated function is Vec::new, which creates a new empty vector (Vec is the vector type in Rust)
    // associated functions are not to be confused as methods, which are also defined in the context of a struct, but are called using the syntax instance_name.method_name (like s2.len())
    // Rust works like this: namespace::StructName::associated_function_name for associated functions, and instance_name.method_name for methods
    // namespace is like a package in Go
    println!("{} world!", s2);

    s2.push_str(", world!"); // push_str is a method that appends a string slice to a String (s2 is mutable, so we can use this method)

    println!("{}", s2); // this will print "hello, world!"

    // string literal contents are known at compile time (they are hardcoded in the binary/executable). That's why string literals are fast
    // while String type contents are known at runtime (they are allocated on the heap).
    // The reason they're allocated on the heap and not pushed to the stack is because they can grow and shrink in size, and the heap is better for that kind of data.
    // The stack is better for data that has a fixed size known at compile time (like integers, floats, etc.)

    // how variables and data interacting with move

    // this is fine because integers are simple values with a known, fixed size at compile time (declared by the type system),
    // so they are stored on the stack, and when we assign x to y, we are copying the value of x into y (a shallow copy, but since it's a simple value, it's effectively a deep copy),
    // so x is still valid after this line, and y is a new variable that has the same value as x
    let mut x = 5;
    let mut y = x;
    x -= 1;
    y += 1;
    println!("x: {}, y: {}", x, y);

    // if we try this with String, it won't work due to the way Rust handles ownership and moves
    // basically we move the owner of the contents of "String::from("hello")" from s3 to s4, so s3 is no longer valid after this line,
    // and s4 is the new owner of the String data. s4 is not a copy of s3, but rather a move of ownership
    let s3 = String::from("hello");

    println!("s3: {}", s3); // this will print "hello"

    let s4 = s3;

    // println!("s3: {}, s4: {}", s3, s4); // this will not compile because s3 is no longer valid
    println!("s4: {}", s4); //

    // how come the println! macro can still print s3 after we moved it to s4?
    // this is because we have yet to "move" s3 to s4
    // but after we assign s3 to s4, s3 is no longer valid, and we can't use it anymore
    // s3 still exists, its still in scope, but its just no longer valid and cannot be mutated/used in any way

    // in order to deep copy the contents of s3 into s4, we can use the clone method, giving ownership of that new data to s4,
    // while s3 still retains ownership of the original data

    let s3 = String::from("hello");
    let s4 = s3.clone(); // this will create a deep copy of the data

    println!("s3: {}, s4: {}", s3, s4);

    let s1 = gives_ownership(); // gives_ownership moves its return value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3

    println!("s1: {}, s3: {}", s1, s3); // s2 is no longer valid, but s3 is valid and has the value that was originally in s2

    // println!("s2: {}", s2); // this will not compile because s2 is no longer valid
}

fn gives_ownership() -> String {
    use std::string::String;

    let some_string = String::from("hello");
    some_string // this will move the ownership of some_string to the caller of this function
    // thus, some_string is basically returned from this function, but instead of copying the value, we are moving the ownership of the value to the caller
    // and some_string is no longer valid after this line, because its ownership has been moved to the caller
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // this will move the ownership of a_string to the caller of this function
}
