use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess the number! Please input your guess:");

        // We initialize guess to an empty instance of a String
        // String is provided by the std library
        // It is growable, and stored on the heap
        // ::new is an associated function of the String type
        // We always prefix an associated function call with ::
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // If we are printing a variable, it can go inside the curly brackets
        // If we are printing the result of an expression, it is placed
        // after the string, separated by a comma
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "YOU WIN!".green());
                break;
            }
        }
    }
}
