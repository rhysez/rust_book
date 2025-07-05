fn main() {
    // Structs allow us to group related pieces of data together
    // Each struct you define is it's own type
    // Similar to tuples, each value in a struct can be a different type
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // To use a struct, we must create an instance of it
    // We must pass in a value for each field, but also the name of the field
    // This makes it clear what each value represents when instantiating the struct
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // To access a specific value from an instance, we use dot notation
    // If the instance is mutable, we can re-assign a value using dot notation
    let mut user2 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");

    // Creating a new instance of a struct is an expression
    // Because it returns a value - the new struct instance
    // We can use this to implicitly return a struct from a function
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username: username,
            email: email,
            sign_in_count: 1,
        }
    }

    // We can simplify this function using the Field Init shorthand
    // Notice 'username' and 'email' are implicitly assigned from the function parameters.
    fn field_init_example(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    // Sometimes we might want to create a new instance from another existing instance
    // Which includes most of the values from the existing instance, but changes some
    // We can achieve this using the Struct Update syntax
    // This is very similar to the spread operator in JavaScript/TypeScript
    // It's worth pointing out that instantiating a struct uses the assignment operator '='
    // This is because we're moving values into user3 from user1.
    // Meaning user1's email and username are no longer valid
    // However, user's active and sign_in_count ARE valid.
    // active and sign_in_count both implement the Copy trait, so they are copied instead of moved.
    // They are stored on the stack because they have fixed values known at compile time.
    // Stack data can be copied.
    let user3 = User {
        email: String::from("yet_another@example.com"),
        // Spreads the remaining values from user1 into this instance
        ..user1
    };

    // Tuple structs
    // These are useful when we want to make a tuple it's own type
    // Which also comes with the added meaning of a name
    struct Color(i32, i32, i32);

    let black = Color(0, 0, 0);

    // You can also define structs without fields, called unit-like structs
    // Useful for when we need to implement a trait on some type
    // But don't have any data you want to store in the type itself
    struct AlwaysEqual;

    let subject = AlwaysEqual;

    // OWNERSHIP IN STRUCTS - IMPORTANT
    // In the User struct, we used String types rather than the &str reference
    // This was deliberate. Using String ensures the struct always owns that data, it cannot be copied elsewhere.
    // If we did use a &str reference, we would need to specify a lifetime on the original &str - how long the &str would be valid for.
    // Rust doesn't know how long that &str would live for, whereas a String will always live as long as the struct that owns it.
    // This is a memory safety guarantee. We do not want any possibility of accessing data that is no longer valid.
    // We will learn about lifetimes in chapter 10.
}
