// create a Rectangle struct
// 'derive' allows us to add custom behavior to our custom types in the form of Traits
// adding #[derive(Debug)] implements the Debug trait to Rectangle
// which allows us to print Rectangle in debug mode
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// The impl keyword allows us to implement a method on a struct
// area() must take 'self' as the first param, which represents the struct it has been implemented on
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        other_rect.width < self.width && other_rect.height < self.height
    }

    // Here we can also implement a function without the &self parameter
    // This is no longer considered a method. It is now an 'associated function'.
    // However, associated functions like this can be used to return a new instance of a type
    // We can call these functions with the :: syntax, e.g rect1::square(10)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // create a new instance of Rectangle
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Using the specifier :? tells println!() we want to use an output format called Debug
    // This allows us to print values that don't implement the Display trait.
    // The Display trait allows the value to be easily displayed in the terminal for end user consumption.
    // However, structs don't implement this trait, as it's not clear to Rust how a struct should be printed.
    println!("rect1 is {rect1:?}");

    // pass an immutable reference to rect1 into area()
    // which means we can continue to use rect1 in this scope
    println!("rect1 is {} square pixels.", rect1.area());
}
