#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn run() {
    // using variables to calculate area of a rectangle
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels for a rectangle with a width of {} and a height of {}.",
        area(width1, height1),
        width1,
        height1
    );

    // refactoring to a tuple
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels for the tuple {:?}.",
        area_tuple(rect1),
        rect1
    );

    // refactoring to a struct
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels for {:?}.",
        area_struct(&rect1),
        rect1
    );

    // using dbg! to print debug information (and return the value of the expression)
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
