fn main() {
    // if statements
    let num = 3;

    if num < 5 {
        println!("{} is less than 5", num);
    } else if num == 5 {
        println!("{} is equal to 5", num);
    } else {
        println!("{} is greater than or equal to 5", num);
    }

    // if as an expression
    let some_condition = true;
    let num = if some_condition { 10 } else { 20 };

    println!("The value of num is: {}", num);

    // loop (infinite loop, like doing while true)

    let mut i = 0;

    // loop == while true
    loop {
        if i == 3 {
            break; // exit the loop when i is 3
        }

        println!("This will print 3 times");

        i += 1; // increment i to eventually break the loop
    }

    // returning values from loops

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // break acts as a return statement, returning the value of counter * 2 when breaking the loop
            break counter * 2; // return the value of counter * 2 when breaking the loop
        }
    };

    println!("The result is: {}", result);

    // below is the same without the loop returning a value, we would have to declare result as mutable and assign it inside the loop

    let mut counter = 0;

    loop {
        counter += 1;

        if counter == 10 {
            counter *= 2; // assign the value to result when breaking the loop
            break;
        }
    }

    println!("The result is: {}", counter);

    // disambiguating between multiple loops with labels

    let mut count = 0;
    'outer_loop: loop {
        println!("count = {count}");
        let mut remaining = 10;

        'inner_loop: loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break 'inner_loop; // break out of the inner loop when remaining is 9
            }
            if count == 2 {
                break 'outer_loop; // break out of the outer loop when count is 2
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // streaming conditionals with while let

    let mut num = 3;

    while num != 0 {
        println!("{}!", num);
        num -= 1;
    }

    println!("LIFTOFF!!!");

    // looping through a collection (compound data type)

    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut i = 0;

    while i < 5 {
        println!("the value is: {}", a[i]);
        i += 1;
    }

    // the above can be rewritten with a for loop, which is more concise and less error-prone

    for element in a {
        println!("the value is: {}", element);
    }

    // ranges with for loops (exclusive ends with .., inclusive ends with ..=)
    let num = 3;

    for number in 1..num {
        println!("{}!", number);
    }

    println!();

    for number in 1..=num {
        println!("{}!", number);
    }

    println!();

    for number in 1..4 {
        println!("{}!", number);
    }

    println!();

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    // if you want to loop through a collection and need the index, you can use the enumerate method
    // it converts the collection into an iterator that yields pairs of (index, value) with the enumerate method
    for (index, value) in a.iter().enumerate() {
        println!("the value at index {} is: {}", index, value);
    }

    // if you want to do something like i += 2 in other languages
    // you can use the step_by method to specify the step size for the loop
    for number in (1..10).step_by(2) {
        println!("{}!", number);
    }

    println!();

    // can also use ranges as iterators directly, which is what the for loop does under the hood
    use std::ops::Range;

    let range: Range<i32> = 1..5; // the type of range is Range<i32> by default, which is an iterator that yields values from 1 to 4

    // in order to use the range as an iterator, we can call the into_iter method on it, which converts it into an iterator that yields values from 1 to 4
    for number in range.into_iter() {
        println!("{}!", number);
    }

    println!();

    // this still works because the for loop automatically calls into_iter on the range,
    // so we can just use the range directly as an iterator without calling into_iter explicitly
    let range: Range<i32> = 1..5; // the type of range is Range<i32> by default, which is an iterator that yields values from 1 to 4

    for number in range {
        println!("{}!", number);
    }

    println!();

    // in order for ranges to be inclusive, we can use the RangeInclusive type, which is created with the ..= syntax
    use std::ops::RangeInclusive;

    let range: RangeInclusive<i32> = 1..=5; // the type of range is RangeInclusive<i32> by default, which is an iterator that yields values from 1 to 5 exclusively

    for number in range {
        println!("{}!", number);
    }

    println!();

    // whats the difference between a iterator and a range? a range is a specific type of iterator that yields values in a sequence,
    // while an iterator is a more general concept that can yield any type of value, not just numbers in a sequence.
    // an iterator can be created from a variety of data structures, such as arrays, vectors, and hash maps,
    // while a range is specifically designed to yield values in a sequence.
}
