fn main() {
    // vector
    let v: Vec<i32> = Vec::new();

    println!("{:?}", v);

    // can also create a vector like thsi
    let v = vec![1, 2, 3];

    println!("{:?}", v);

    // updating a vector with push (like append in python)
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    println!("{:?}", v);

    // reading elements
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2); // get the element at 2nd index
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // the following below wont work
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {}", first);
    // this is because the reference to the first element would be pointing to deallocated memory

    // iterating over values in a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // using enum
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);

    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v

        println!("{:?}", v)
    } // <- v goes out of scope and is freed here

    // can create a string with to_string method

    let data = "initial contents";

    let s = data.to_string();

    println!("{}", s);

    // The method also works on a literal directly:
    let s = "initial contents".to_string();

    println!("{}", s);

    // strings are UTF-8 encoded
    let hello = String::from("नमस्ते");
    println!("{}", hello);

    // updating a string
    let mut s = String::from("foo");
    s.push_str("bar");

    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // push takes in a single character and adds to the String
    let mut s = String::from("lo");
    s.push('l');

    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // can use format! too
    let s = format!("{s1}-{s2}-{s3}");

    println!("{}", s);

    // rust strings dont support indexing
    // this is because of the way strings are stored in memory, and the fact that some characters can be represented by
    // multiple bytes in UTF-8 encoding

    // a string is a wrapper over a Vec<u8>
    // what thsi means is that a string is a collection of bytes, and not necessarily a collection of characters
    // this is because some characters can be represented by multiple bytes in UTF-8 encoding

    let hello = String::from("Здравствуйте");

    let s = &hello[0..4]; // can technically range index a string

    println!("{}", s); // prints Зд, not Здра. this is because the first character З is represented by 2 bytes, and the second character д is also represented by 2 bytes

    // iterating over the characters in a string
    for c in "Зд".chars() {
        println!("{c}");
    }

    // iterating over the bytes in a string
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
