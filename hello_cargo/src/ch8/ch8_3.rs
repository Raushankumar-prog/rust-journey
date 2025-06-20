// Chapter 8.3: Storing Keys with Associated Values in Hash Maps - Notes & Examples

pub fn ch8_3() {
    // 1. What is a HashMap?
    // ---------------------
    // - HashMap<K, V>: Stores key-value pairs using a hash function.
    // - Keys: Any type, usually implements Eq and Hash.
    // - Values: Any type.
    // - Not included in Rust prelude; bring into scope with:
    use std::collections::HashMap;

    // 2. Creating & Inserting
    // -----------------------
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 3. Accessing Values
    // -------------------
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // .get() returns Option<&V>. Use .copied() if V: Copy, and .unwrap_or(default) to handle missing keys.

    // 4. Iterating
    // ------------
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // 5. Ownership
    // ------------
    // - Inserted owned types (e.g., String) are moved into the map.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid after insertion.

    // 6. Updating Hash Maps
    // ---------------------
    // (a) Overwriting a value (existing key)
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // Value replaced, now "Blue": 25

    // (b) Only insert if key is absent (entry API)
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    // Now: {"Blue": 10, "Yellow": 50}

    // (c) Update a value based on the old value (word count example)
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}"); // e.g., {"world": 2, "hello": 1, "wonderful": 1}

    // 7. Hashing Functions
    // --------------------
    // - Default: SipHash, secure but not fastest.
    // - Can use custom hasher by specifying a type that implements BuildHasher.

    // 8. Summary of Methods
    // ---------------------
    // - insert, get, contains_key, remove, entry, keys, values, iter, len, is_empty, clear, etc.

    // 9. Common Exercises (see summary)
    // ---------------------------------
    // - Median/mode of a list (vector + hash map)
    // - Pig Latin conversion (string manipulation)
    // - Company directory by department (hash map + vectors)

    println!("See source for notes and examples on HashMap<K, V> in Rust.");
}