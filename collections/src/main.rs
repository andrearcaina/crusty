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
}
