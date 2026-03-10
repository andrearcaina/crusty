#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

pub fn run() {
    let coin = Coin::Penny;
    println!("Value of {:?} is {}", coin, value_in_cents(&coin));

    let coin = Coin::Quarter(UsState::Alaska);
    println!("Value of {:?} is {}", coin, value_in_cents(&coin));

    // lets say we want to write a function that takes an Option<i32> and, if theres a value, adds 1 to it

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six: {:?}", six);
    println!("none: {:?}", none);

    // using enums, we can also take special actions for a few particular values, but for all other vaues take the default action
    // the following code shows how to match on a single value and execute code based on that value
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // we can also use match to handle _ as a catch-all pattern for all other values
    let dice_roll = 12;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => move_player(dice_roll),
    }

    // can also make nothing happen for a particular pattern
    let dice_roll = 5;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => {} // do nothing for all other values, u can also use _ => (),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1), // does Some(5) match Some(i)? indeed it does. The i binds to the value contained in Some, so i takes the value of 5
    }
}

fn add_fancy_hat() {
    println!("Adding fancy hat!");
}

fn remove_fancy_hat() {
    println!("Removing fancy hat!");
}

fn move_player(num_spaces: u8) {
    println!("Moving player {} spaces!", num_spaces);
}
