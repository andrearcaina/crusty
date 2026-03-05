#[derive(Debug)]
struct Square {
    width: u32,
    height: u32,
}

impl Square {
    // constructor
    fn new(size: u32) -> Self {
        // can return Self because we are inside the impl block for Square, can also return Square
        Self {
            width: size,
            height: size,
        }
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }
}

// can have multiple impl blocks for the same struct, this is just for organization
impl Square {
    fn area(&self) -> u32 {
        if self.width != self.height {
            return 0; // Return 0 to indicate that this is not a square
        }

        self.width * self.height
    }

    fn can_hold(&self, other: &Square) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn run() {
    // will panic because the width and height are not equal
    let sq1 = Square {
        width: 30,
        height: 50,
    };

    if sq1.area() == 0 {
        println!("Warning: sq1 width and height are not equal. This is not a square.");
    } else {
        println!("The area of sq1 is {} square pixels.", sq1.area());
    }

    let sq2 = Square {
        width: 40,
        height: 40,
    };

    println!("The area of sq2 is {} square pixels.", sq2.area());
    println!("The width of sq2 is {} pixels.", sq2.width());
    println!("The height of sq2 is {} pixels.", sq2.height());

    let sq3 = Square {
        width: 20,
        height: 20,
    };

    println!("Can sq2 hold sq3? {}", sq2.can_hold(&sq3));

    let sq4 = Square::new(25);

    println!("The area of sq4 is {} square pixels.", sq4.area());
}
