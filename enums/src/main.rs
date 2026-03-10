mod r#match; // match is a reserved keyword, so we use r#match instead (this escapes the keyword)

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// can have structs with enum variants as fields
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// however, representing the same concept using just an enum is more concise
// rather than an enum inside a struct, we can put data directly into the enum variants
#[derive(Debug)]
enum IpAddrEnum {
    V4(String),
    V6(String),
}

// another advantage to using an enum over a struct
// each variant can have different types and amounts of associated data
#[derive(Debug)]
enum IpAddrEnum2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,                       // a unit struct in an enum variant
    Move { x: i32, y: i32 },    // a struct in an enum variant
    Write(String),              // a string in an enum variant
    ChangeColor(i32, i32, i32), // a tuple struct in an enum variant
}

// the above enum is more concise than the separate structs below
// struct Quit;
// struct Move { x: i32, y: i32 };
// struct Write(String);
// struct ChangeColor(i32, i32, i32);

// can also do impl with enums
impl Message {
    fn call(&self) {
        println!("called {:?}", self);
    }
}

// option enum
#[derive(Debug)]
enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;

    println!("{:?}, {:?}", v4, v6);

    route(v4);
    route(v6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    println!("{:?}", home);

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:?}", loopback);

    let home = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopback = IpAddrEnum::V6(String::from("::1"));
    println!("{:?}, {:?}", home, loopback);

    let home = IpAddrEnum2::V4(127, 0, 0, 1);
    let loopback = IpAddrEnum2::V6(String::from("::1"));
    println!("{:?}, {:?}", home, loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    // option enum usage
    let x: Option<i32> = Option::Some(5);
    let y: Option<i32> = Option::None;
    println!("{:?}, {:?}", x, y);

    // basically option is a way to represent optional values. what this means
    // is that a value can either be present (Some) or absent (None)
    // it's like doing x = None in python, then changing it to x = 5 on some condition
    // or like in typescript u can do let x: number | undefined = undefined;
    // T is also the generic type parameter for Option<T>, meaning it can hold any type
    // like in go, u can do var x any = nil or var x interface{} = nil

    // the following cannot be done with Option<T> in rust
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;
    // this wont compile because Option<i8> is not a numeric type

    // can also use match expressions with enums
    r#match::run();
}

// can now have functions that take in IpAddrKind enum variants
fn route(ip_kind: IpAddrKind) {
    println!("{:?}", ip_kind)
}
