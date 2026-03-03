fn main() {
    // for the purpose of this, we are assuming only ASCII in this section

    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    println!("word: {}", word);
    println!("first word: {}", &s[..word]); // prints the first word

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but s no longer has any content that we
    // could meaningfully use with the value 5, so word is now totally invalid!
    // so, println!("word: {}", word); still works, but it's a bug waiting to happen if we try to use s with word
    println!("word: {}", word);

    // use first_word_slice instead of just returning the index
    let word_slice = first_word_slice(&s);
    println!("first word slice: {}", word_slice);

    // function param takes in a str literal, so we can pass in a &str directly
    let s = String::from("hello world ");

    println!("first word str: {}", first_word_str(&s));
    println!("first word str: {}", first_word_str(&s[..6]));
    println!("first word str: {}", first_word_str(&s[6..]));
    // note, &s[..] is the entire string, and is the same as &s (it's a reference to the entire string, not a copy)
    println!("first word str: {}", first_word_str(&s[..]));

    // slices can be used with integer arrays as well
    let numbers = [1, 2, 3, 4, 5];
    let slice = &numbers[1..3];
    println!("slice: {:?}", slice); // {:?} is a debug format specifier for printing the slice,
    println!("slice: {:#?}", slice); // {:#?} is a pretty-printed debug format specifier
    // {:?} can be used with any type that implements the Debug trait
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // the s String, as a bytes slice

    // iterate through each idx and byte
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            // find a space
            return i; // once space found, return that index
        }
    }

    // otherwise return length of string
    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes(); // the s String, as a bytes slice

    // iterate through each idx and byte
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            // find a space
            return &s[..i]; // once space found, return string slice up to that index
        }
    }

    // otherwise return the entire string
    &s // could also return &s[..], but &s is more idiomatic for returning the entire string
}

// its actually better to take a &str slice instead of a &String, because we can pass in a &str directly
// this avoids the need to allocate a new String on the heap
fn first_word_str(s: &str) -> &str {
    let bytes = s.as_bytes(); // the s String, as a bytes slice

    // iterate through each idx and byte
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            // find a space
            return &s[..i]; // once space found, return string slice up to that index
        }
    }

    // otherwise return the entire string
    &s
}
