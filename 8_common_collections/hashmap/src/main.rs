use std::collections::HashMap;

fn main() {
    // HashMaps are a collection of key-value pairs.
    // They are similar to dictionaries in Python or objects in JavaScript.
    let mut scores = HashMap::new();

    // Inserting two key-value pairs into the HashMap.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 40);

    let team_name = String::from("Blue");

    // We can retrieve a value from the HashMap using its key.
    let score = scores.get(&team_name).copied().unwrap_or(0);
}
