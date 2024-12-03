// We use the derive keyword to implement the the Debug trait on our struct
// This allows us to print out more helpful debugging info.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// An 'implementation' block for our Rectangle struct.
// Everything inside of this impl will be associated with our Rectangle struct.
// Here, we define area() as a method on the Rectangle struct.
// Methods take in a parameter 'self' which is a reference to the struct it is being called on.
// We access methods via dot notation (e.g. foo.area())
// If we want to change anything about our Rectangle, we can use a mutable reference: &mut self.
// Functions defined inside of an impl block, that don't take in 'self', are called Associated Functions.
// Associated functions are often used for constructors that return a new instance of the struct.

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function.
    // Self refers to the type of the struct the impl block is for.
    // We use square() as a constructor function to create a square Rectangle.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let square1 = Rectangle::square(30);

    if rect1.width() {
        println!("The rectangle width is {}", rect1.width);
    }
    println!("The area of the rectangle is {}", rect1.area());

    if square1.width() {
        println!("The square width is {}", square1.width);
    }
}