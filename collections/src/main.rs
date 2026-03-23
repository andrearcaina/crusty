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

    // hash set
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(1); // same as python set.add()
    set.insert(2);
    set.insert(3);
    println!("set: {:#?}", set);
    set.remove(&1); // same as python set.discard()
    println!("set after remove: {:#?}", set);

    // check if a value is in the set (O(1) lookup)
    if set.contains(&2) {
        println!("set contains 2");
    }

    // hash map
    use std::collections::HashMap;
    let mut map = HashMap::<String, i32>::new();
    map.insert(String::from("one"), 1);
    map.insert(String::from("two"), 2);
    map.insert(String::from("three"), 3);
    println!("map: {:#?}", map);
    map.remove("one");
    println!("map after remove: {:#?}", map);

    // accessing values in a hash map
    let val = map.get(&String::from("two")).copied().unwrap_or_default(); // copies the value out of the map, or returns 0 if the key is not found
    println!("two: {}", val);

    for (key, val) in &map {
        println!("{}: {}", key, val);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut fav = HashMap::new();
    fav.insert(field_name, field_value);
    println!("map: {:#?}", fav);

    // cannot use field_name and field_value after insert, as they are moved into the map

    let mut map = HashMap::<String, i32>::new();
    map.insert(String::from("Green"), 20);
    map.insert(String::from("Blue"), 50);
    // updating a value in a hash map
    map.insert(String::from("Blue"), 100);
    println!("map after update: {:#?}", map);

    // adding a key and val only if a key isn't present
    map.entry(String::from("Yellow")).or_insert(30); // inserts 30 if "Yellow" is not already in the map
    map.entry(String::from("Green")).or_insert(30); // does nothing, as "Green" is already in the map (but, the value is not updated since it already has a value)
    println!("map after insert: {:#?}", map);

    // updating a value based on the old value
    if let Some(val) = map.get_mut(&String::from("Blue")) {
        *val += 100;
    }
    println!("map after update based on old value: {:#?}", map);

    let text = "Orange Red Brown Orange Orange Orange";

    for word in text.split_whitespace() {
        let count = map.entry(String::from(word)).or_insert(0);
        *count += 1;
    }

    println!("map after word count: {:#?}", map);

    // check if a key is in the map
    if map.contains_key(&String::from("Green")) {
        println!("map contains Green");
    }

    // get a value from the map using let Some(val)
    if let Some(val) = map.get(&String::from("Blue")) {
        println!("map value for Blue: {}", val);
    }

    // get a value without using let Some(val)
    let val = map.get(&String::from("Blue")); // this doesn't copy the value, it just borrows it (it returns an Option<&T>)
    println!("map value for Blue (borrowed): {:?}", val);

    // you have to copy the value or use let Some(val) to get ownership
    let val = val.copied().unwrap_or_default();
    println!("map value for Blue (copied): {:?}", val);
}
