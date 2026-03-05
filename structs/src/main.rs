use std::string::String;
mod example;
mod methods;

#[derive(Debug)] // this allows us to print the struct using {:?} or {:#?}
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(i32, i32, i32); // tuple struct
#[derive(Debug)]
struct Point(i32, i32, i32); // tuple struct

#[derive(Debug)]
struct AlwaysEqual; // unit struct

// manually implement PartialEq for AlwaysEqual, so that all instances of AlwaysEqual are equal to each other
impl PartialEq for AlwaysEqual {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("User1 struct: {:#?}", user1);

    user1.active = false;
    user1.email = String::from("anotheremail@example.com");

    println!("Updated User1 struct: {:#?}", user1);

    let mut user2 = build_user("anotherusername456", "anotheremail@example.com");

    println!("User2 struct: {:#?}", user2);

    user2.sign_in_count += 1;
    user2.username = String::from("updatedusername789");

    println!("Updated User2 struct: {:#?}", user2);

    // can use ... to create a new instance of the struct with some fields from another instance
    let user3 = User {
        email: String::from("3rdemail@example.com"),
        ..user1
    };

    println!("User3 struct: {:#?}", user3);

    // rust also has tuple structs, that look similar to tuples, but are different
    // they are useful when you want to give a name to a tuple, but don't need to name each field
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black color: {:#?}", black);
    println!("Origin point: {:#?}", origin);

    // can also have unit structs, which are useful when you need to implement a trait on a type that has no data
    let subject = AlwaysEqual;

    println!("Subject: {:#?}", subject);

    let subject1 = AlwaysEqual;
    let subject2 = AlwaysEqual;

    println!("Are subject1 and subject2 equal? {}", subject1 == subject2);
    println!(
        "Are subject1 and subject2 not equal? {}",
        subject1 != subject2
    );

    // examples of using structs (check example.rs)
    example::run();

    // examples of using methods (check methods.rs)
    methods::run();
}

fn build_user(email: &str, username: &str) -> User {
    User {
        active: true,
        username: String::from(username), // String::from creates a new String from a string slice (&str)
        email: email.to_string(), // while to_string converts the string slice into a String (if the variable implements the ToString trait)
        sign_in_count: 1,
    }
}
