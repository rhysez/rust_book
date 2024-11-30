// The stack is fixed-size and cannot grow/shrink during runtime.
// When we compile a Rust program, the stack size is calculated. So at runtime, it cannot change.
// When we call a function, that function is added to the stack, as a "stack frame".
// It is then popped off the stack and all of it's local variables are dropped.
// On the stack, we store values of known, fixed sizes.

// The heap is dynamic in size, and can store values of unknown sizes at compile-time.
// On the heap we store values of unknown, potentially very large sizes.
// To access a value on the heap, we must use a pointer, which points to the value's memory address.

// Accessing data on the stack is always faster than on the heap, because we don't need to follow a pointer.
// Also, storing data on the stack is faster than on the heap, because we don't spend time finding memory.



fn main() {
    println!("Hello, world!");
}
