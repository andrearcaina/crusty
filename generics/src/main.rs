fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // with a function that uses generics and the PartialOrd trait, we can find the largest item in a list of any type that implements the PartialOrd trait
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // can also use generics to define a struct that can hold any type of data
    // in this example, we define a struct Point that has two fields, x and y, which can be of any type T
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    println!("Integer point: {:?}", integer_point);
    println!("Float point: {:?}", float_point);

    // the following Point would not compile because the fields x and y have different types (i32 and f64)
    // let invalid_point = Point { x: 5, y: 4.0 };
    // this is because the struct definition requires that both fields have the same type T, and in this case, they do not
    // but we could define a struct that allows for different types for x and y using multiple type parameters, like this:

    #[derive(Debug)]
    struct Point2<T, U> {
        x: T,
        y: U,
    }

    let point_2 = Point2 { x: 5, y: 4.0 };

    println!("Point 2: {:?}", point_2);

    // enums can also use generics to define variants that can hold different types of data
    #[derive(Debug)]
    enum Option<T> {
        Some(T),
        None,
    }

    let some_number = Option::Some(5);
    let some_char = Option::Some('a');
    let none_number: Option<i32> = Option::None; // can be Option<i32> or Option<char> or any other type, as long as it matches the type parameter T

    println!("Some number: {:?}", some_number);
    println!("Some char: {:?}", some_char);
    println!("None number: {:?}", none_number);

    // in method definitions
    // we can also use generics in method definitions to allow methods to operate on different types of data
    #[derive(Debug)]
    struct Point3<T> {
        x: T,
        y: T,
    }

    impl<T> Point3<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let point = Point3 { x: 5, y: 10 };
    println!("p.x = {}", point.x());

    #[derive(Debug)]
    struct Point4<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> Point4<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point4<X2, Y2>) -> Point4<X1, Y2> {
            // takes the original point x vaue and the other point y value and creates a new Point4 with those values,
            // and the types of the new Point4 are determined by the types of the original points
            Point4 {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point4 { x: 5, y: 10.4 };
    let p2 = Point4 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    // at this point forward, p1 and p2 can no longer be used because they have been moved into the mixup method, and the mixup method takes ownership of both p1 and p2

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// we can use a single function to find the largest item in a list, regardless of the type of the list's elements
// this can be done using generics and trait bounds (the PartialOrd trait in this case)
// the function largest takes a slice of any type T that implements the PartialOrd trait, and returns a reference to the largest item in the list
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
