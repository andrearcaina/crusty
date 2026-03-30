pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: u64) -> u64 {
    a + 2
}

fn greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[derive(Debug)]
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Self {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Self { value }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // unit tests on the add function
    #[test]
    fn add_two_plus_two() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_three_plus_three() {
        let result = add(3, 3);
        assert_eq!(result, 6);
    }

    #[test]
    fn add_four_plus_four() {
        let result = add(4, 4);
        assert_eq!(result, 8);
    }

    #[test]
    fn add_five_plus_five() {
        let result = add(5, 5);
        assert_ne!(result, 11);
    }

    #[test]
    fn panic_test() {
        panic!("Make this test fail");
    }

    // unit tests on add_two function
    #[test]
    fn it_adds_two() {
        let result = add_two(5);
        assert_eq!(result, 7);
    }

    #[test]
    fn it_adds_three() {
        let result = add_two(3);
        assert_ne!(result, 6);
    }

    // unit tests on the greeting function
    #[test]
    fn greeting_works() {
        let result = greeting("Alice");
        assert_eq!(result, "Hello, Alice!");
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            // can have customized failure messages
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }

    // unit tests on the Guess struct
    #[test]
    fn guess_works() {
        let guess = Guess::new(50);
        assert_eq!(guess.value, 50);
    }

    #[test]
    // can have a should_panic attribute to test panics as well as customize the expected message
    #[should_panic(expected = "Guess value must be between 1 and 100, got 101")]
    fn guess_out_of_bounds() {
        Guess::new(101);
    }

    // unit tests on the Rectangle struct with can_hold method
    #[test]
    fn rectangle_can_hold() {
        let rect1 = Rectangle {
            width: 10,
            height: 10,
        };
        let rect2 = Rectangle {
            width: 5,
            height: 5,
        };
        assert!(rect1.can_hold(&rect2));
    }

    #[test]
    fn rectangle_cannot_hold() {
        let rect1 = Rectangle {
            width: 5,
            height: 5,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 10,
        };
        assert!(!rect1.can_hold(&rect2));
    }
}
