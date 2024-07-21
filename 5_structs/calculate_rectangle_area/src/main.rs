// create a Rectangle struct
// 'derive' allows us to add custom behavior to our custom types in the form of Traits
// adding #[derive(Debug)] implements the Debug trait to Rectangle
// which allows us to print Rectangle in debug mode
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // create a new instance of Rectangle
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Using the specifier :? tells println! we want to use an output format called Debug
    // This allows us to print values that don't implement the Display trait. Which is usable for debugging structs.
    // The Display trait allows the value to be easily displayed in the terminal for end user consumption.
    // However, structs don't implement this trait, as it's not clear to Rust how a struct should be printed.
    println!("rect1 is {rect1:?}");

    // pass an immutable reference to rect1 into area()
    // which means we can continue to use rect1 in this scope
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

// this function takes an immutable reference to the Rectangle struct
// we can also see that the width and height are clearly related to eachother
// as they are both derived from the Rectangle struct
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
