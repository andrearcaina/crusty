#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    // returns the user's preferred shirt color if available, otherwise returns the most stocked color
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        if let Some(preference) = user_preference {
            preference
        } else {
            self.most_stocked()
        }
    }

    fn giveaway_closure(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked()) // closure that takes no arguments, captures &self, and returns a ShirtColor
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    println!("Closures");

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
    // closures are like this
    // fn add_one_v1(x: u32) -> u32 { x + 1 } // this is a function that takes x and returns x + 1
    // let add_one_v2 = |x: u32| -> u32 { x + 1 }; // this is a closure that takes x and returns x + 1 (basically same as add_one_v1)
    // with this info we can rewrite the giveaway method as a closure
    let giveaway3 = store.giveaway_closure(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway3
    );

    // another example
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list); // could also wrap println! with { }

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // we can take this up a notch with a mutable list
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}");

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // same as doing list.sort(key=lambda x: x[0]) in python
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    // iterators

    println!("Iterators");

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
    for val in v1_iter {
        // another way of doing v1_iter.next(), or for val in v1 {}
        println!("Got: {val}");
    }

    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    println!("{:?}", v1_iter.next()); // Some(&1)
    println!("{:?}", v1_iter.next()); // Some(&2)
    println!("{:?}", v1_iter.next()); // Some(&3)
    println!("{:?}", v1_iter.next()); // None

    let total: i32 = v1.iter().sum();
    println!("Total: {total}");

    let v1: Vec<i32> = vec![1, 2, 3];
    // converts v1 into an iterator, then maps each element to x + 1 (using the closure |x| x + 1), then collects the result into v2
    // collect() converts the iterator into a collection, in this case a Vec<i32>
    let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);
}
