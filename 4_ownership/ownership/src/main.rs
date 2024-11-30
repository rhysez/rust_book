// ---MEMORY---
// The stack is fixed-size and cannot grow/shrink during runtime.
// When we compile a Rust program, the stack size is calculated. So at runtime, it cannot change.
// When we call a function, that function is added to the stack, as a "stack frame".
// It is then popped off the stack and all of it's local variables are dropped.
// On the stack, we store values of known, fixed sizes.

// The heap is dynamic in size, and can store values of unknown sizes at compile-time.
// On the heap we store values of unknown, potentially very large sizes.
// To access a value on the heap, we must use a pointer, which points to the value's memory address.

// Accessing data on the stack is always faster than on the heap, because we don't need to follow a pointer.
// Also, storing data on the stack is faster than on the heap, because we don't spend time allocating memory.

// ---OWNERSHIP---
// 1. Each value in Rust has only one owner.
// 2. When the owner goes out of scope, the value is dropped.
// The points above protect us from accidentally forgetting to de-allocate memory.
// This is known as Rust making a 'memory safe guarantee'.

// ---REFERENCES---
// A reference is like looking through a window at value, rather than using it directly.
// In code, a reference is denoted by prefixing a variable or type with &.
// References allow us to use values without taking ownership.
// A reference simply points to the original pointer (its a pointer to a pointer).

// ---MUTABLE REFERENCES---
// We can mutate values without taking ownership, by using a mutable reference.
// First, we make our variable mutable by adding 'mut' to the declaration.
// Then, we reference the variable like so: &mut
// If we mutate this reference, the original value will also be mutated.
// However, there can only ever be one mutable reference at one time, to prevent data races.

fn main() {
    { // new scope, s not yet declared
        let s = String::from("hello"); // s is now valid
    } // scope ends, s is dropped


    let x = 5; // x is a fixed value stored on the stack.
    let y = x; // x is copied into y.

    let s1 = String::from("hello"); // s1's size is unknown at compile time, stored on heap.
    let s2 = s1; // s1 is moved into s2 (not copied).

    let s3 = String::from("hello");
    let len = calculate_length(&s3); // We pass a reference to s3 so the value isn't moved.

    println!("The length of '{}' is {}.", s3, len);
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}
