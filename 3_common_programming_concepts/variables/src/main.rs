fn main() {
    // Scalar types
    // these represent a single value

    // variables in rust are immutable by default
    // we can add the 'mut' keyword to make it mutable
    let x: u32 = 5;

    // here we create a new variable with the same name as the previous
    // the rust compiler will see and calculate the value of this variable first
    // this is called shadowing
    // this allows us to perform operations on an immutable variable
    // and then it will be immutable after those operations have been completed
    let x: u32 = x + 1;

    {
        let x: u32 = x * 2;
        println!("The value of x is: {x}");
    }

    println!("The value of x is: {x}");

    // floating-point type
    // rust's floating-point types are f32 and f64
    let y: f64 = 2.4;

    println!("The value of y is: {y}");

    // boolean type
    let my_bool: bool = false;

    println!("The value of my_bool is {my_bool}");

    // char type
    // represents a single character, must be in single quotes
    let my_char: char = 'z';

    println!("The value of my_char is {my_char}");

    // Compound types
    // These represent a group of values

    // Tuple type
    // A fixed collection of values that can have varying types
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // We can access a value from a tuple by destructuring the tuple
    let (a, b, c) = tup;

    // Or by dot notation
    let first_tup_value = tup.0;

    println!("The value of a is {a}");

    // Array type
    // Fixed length, all values must be of the same type
    // Arrays are useful when we want our data to be stored on the stack
    // We can annotate an array via this format [Type, Number of values]
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    // If all values in an array are the same, we can initialize it like this
    // This is the same as writing [3, 3, 3, 3, 3]
    let my_array_2 = [3; 5];

    // We can access a value from an array like this
    // This will return 1
    // Attempting to access an index past the end of the array will cause
    // the program to panic at runtime
    let one: i32 = my_array[0];
}
