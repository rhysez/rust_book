
// Vectors store multiple values next to eachother in memory. 
// This is different to an array, where the values appear in random memory locations. 
fn main() {
    // Vectors are defined using the Vec<T> type, where T is the type of the values stored in the vector.
    // We can create a new vector using the Vec::new() method, or the vec! macro.
    // The Vec::new() method creates an empty vector, while the vec! macro creates
    // a new vector with the values provided in the macro.
    // The type of the vector is inferred from the values provided in the macro.
    let mut v: Vec<i32> = Vec::new();
    let _v_inferred = vec![1, 2, 3];

    // We can add a value to a vector using the .push method (the vector must be mutable).
    v.push(1);
    v.push(2);
    v.push(3);

    // We can get a value from a vector using indexing, or the .get method.
    let third: &i32 = &v[2];
    println!("The value of the third element is: {}", third);

    let third: Option<&i32> = v.get(2); // .get returns a reference to the value.

    // We can then use pattern matching to handle potential errors.
    // Some() is returned if the value does exist.
    // None is returned if the value does not exist.
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element.")
    }

    // We can iterate over a vector like this.
    for i in &v {
        println!("{i}");
    }

    // Vectors can only store values of the same type.
    // However, we can use enums to get around this!
    // An enum can store multiple values of different types...
    // However, when stored in the enum, they are all within the enum type.
    // See below, SpreadsheetCell stores an i32, f64 and String...
    // However, they are all technically of type SpreadsheetCell.
    // This means they can safely be stored in the vector. 
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    // Now we have an int, float and string stored in the same vector.
    // Although technically they are all of type SpreadsheetCell.
    let _row = vec![
        SpreadsheetCell::Int(24),
        SpreadsheetCell::Float(12.4),
        SpreadsheetCell::Text(String::from("Hello"))
    ];
}
