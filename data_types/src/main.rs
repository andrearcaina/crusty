fn main() {
    // scalar types
    // there are 4 of them: integers, floating-point numbers, booleans, and characters

    // integer types
    // default is i32, but you can also use i8, i16, i64, i128, u8, u16, u32, u64, u128, isize, or usize
    let x: i8 = 127; // 8-bit signed integer
    let y: u8 = 255; // 8-bit unsigned integer
    let z: isize = 64; // pointer-sized signed integer (this is 64 bits on a 64-bit platform, or 32 bits on a 32-bit platform)
    let w: usize = 64; // pointer-sized unsigned integer (this is 64 bits on a 64-bit platform, or 32 bits on a 32-bit platform)

    println!("x: {}, y: {}, z: {}, w: {}", x, y, z, w);

    // signed integers are stored using two's complement representation, which means that the most significant bit is used to indicate the sign of the number (0 for positive, 1 for negative)
    // for example, the number -1 is represented as 11111111 in an 8-bit signed integer, and the number -128 is represented as 10000000 in an 8-bit signed integer
    // number literals can be written in decimal, hexadecimal, octal, or binary notation, and they can also include underscores for readability
    let a = 98_222; // decimal
    let b = 0xff; // hexadecimal
    let c = 0o77; // octal
    let d = 0b1111_0000; // binary
    let e = b'A'; // byte literal (this is a u8 value that represents the ASCII code for the character 'A')
    println!("a: {}, b: {}, c: {}, d: {}, e: {}", a, b, c, d, e);

    // integer overflow occurs when the value of a variable exceeds the maximum or minimum value that can be stored in its type
    // to explicitly handle the possibility of overflow, rust provides wrapping methods
    let f: u8 = 255;
    let g = f.wrapping_add(1); // this will wrap around to 0 instead of panicking
    println!("f: {}, g: {}", f, g);
    // you can also return the None value instead of panicking by using the checked_add method
    let f: u8 = 255;
    let g = f.checked_add(1); // this will return None instead of panicking
    println!("f: {}, g: {:?}", f, g);
    // you can also return the value and a boolean indicating whether the operation was successful by using the overflowing_add method
    let f: u8 = 255;
    let (g, overflowed) = f.overflowing_add(1); // this will return (0, true) because the value wrapped around to 0 and overflowed
    println!("f: {}, g: {}, overflowed: {}", f, g, overflowed);
    // you can also saturate the value at the maximum or minimum value instead of panicking by using the saturating_add method
    let f: u8 = 255;
    let g = f.saturating_add(1); // this will return 255 because the value is saturated at the maximum value of 255
    println!("f: {}, g: {}", f, g);

    // floating point types
    // default is f64, but you can also use f32

    // notice how this is shadowing. kinda cool
    let x: f32 = 3.14; // 32-bit floating-point number
    let y: f64 = 2.71828; // 64-bit floating-point number
    println!("x: {}, y: {}", x, y);

    // numeric operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!(
        "sum: {}, difference: {}, product: {}, quotient: {}, truncated: {}, remainder: {}",
        sum, difference, product, quotient, truncated, remainder
    );

    // boolean type
    let t = true;
    let f: bool = false; // with explicit type annotation

    println!("t: {}, f: {}", t, f);

    // char type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);

    // Compound Types
    // basically types that can group multiple values into one type.

    // tuples
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tuple; // destructuring the tuple into individual variables
    println!("The value of y is: {}", y);

    // can also access tuple elements directly by index
    let five_hundred = tuple.0;
    let six_point_four = tuple.1;
    let one = tuple.2;

    println!(
        "five_hundred: {}, six_point_four: {}, one: {}",
        five_hundred, six_point_four, one
    );

    // arrays
    let array = [1, 2, 3, 4, 5];
    let first = array[0];
    let second = array[1];

    println!("first: {}, second: {}", first, second);

    // can also specify the type and length of the array
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array: {:?}", array);

    // can also initialize an array with the same value for all elements
    let array: [i32; 5] = [3; 5]; // this creates an array of length 5

    println!("array: {:?}", array);

    // in order to change an element of an array, the array must be mutable
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("array before change: {:?}", array);

    array[0] = 10; // this changes the first element of the array to 10

    println!("array after change: {:?}", array);
}
