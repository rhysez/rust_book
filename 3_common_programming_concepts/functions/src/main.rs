// main is the entry point of all Rust programs

// Statements and expressions:
// Statements are instructions that perform some action and do not return a value
// Expressions evaluate to a resultant value, such as 1 + 2 evaluates to 3

fn main() {
    println!("Hello, world!");
    another_function("programmer");

    // this is a statement, nothing is returned
    let y: i32 = 6;

    // this is an expression, which evaluates to a value of 10
    let x: i32 = y + 4;

    // creating a new scope is considered to be an expression
    let z: i32 = {
        let x = 3;
        // this expression returns 4
        // z is now 4
        // expressions do not end with semicolons
        // if we add a semicolon, it will turn into a statement
        // which will not return a value
        x + 1
    };

    println!("The value of z is {z}")
}

fn another_function(greeting: &str) {
    println!("Hello, {greeting}!");
}

// The value of the final expression in a function is returned
// when the function is called
// For example, five() will simply return 5
fn five() -> i32 {
    5
}
