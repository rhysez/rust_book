fn main() {
    let number_list = vec![10, 32, 54, 23, 42];

    let largest = get_largest(number_list);

    println!("The largest number is {}", largest);

    let char_list = vec!['y', 'z', 's', 'k', 'l'];

    let largest = get_largest(char_list);

    println!("The largest number is {}", largest)
}

// We can use < > after the function name to define a generic type
// Generic types allow us to define multiple argument types for our functions.
// If your function has only 1 generic type, it is conventional to name it 'T'.
// A generic can be any type. For example, if your function might compare both numbers and chars, a generic is helpful. 

// In this function, we're comparing number and largest (number > largest).
// To get around this, we can tell Rust that this generic can only be a type that can be compared (like a integers, chars, etc for example).
// Traits are covered in the next chapter of the book. 
fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number
        }
    }

    largest
}

// We can also use generics in structs, like so.
// This example also demonstrates using multiple generics. 
struct Point<T, U> {
    x: T,
    y: U
}

// This is how we use generics in implementations.
impl<T, U> Point<T, U> {

}