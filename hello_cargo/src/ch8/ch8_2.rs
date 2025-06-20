// Chapter 8.2: Storing UTF-8 Encoded Text with Strings - Notes & Examples

pub fn ch8_2() {
    // 1. What is a "String" in Rust?
    // ------------------------------
    // - String: growable, mutable, owned, UTF-8 encoded text.
    // - &str: string slice, borrowed reference to UTF-8 (e.g. string literals).
    // - Both String and &str are UTF-8 encoded.

    // 2. Creating Strings
    // -------------------
    let mut s = String::new(); // empty String

    let data = "initial contents";
    let s1 = data.to_string();
    let s2 = "initial contents".to_string();
    let s3 = String::from("initial contents");

    // UTF-8: Can store text in any language!
    let hello = String::from("नमस्ते");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hello");

    // 3. Updating/Modifying Strings
    // -----------------------------
    // (a) Appending string slices
    let mut s = String::from("foo");
    s.push_str("bar"); // s == "foobar"

    // (b) push single character
    let mut s = String::from("lo");
    s.push('l'); // s == "lol"

    // (c) Concatenation with +
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 is moved, s3 == "Hello, world!"

    // (d) format! macro (recommended for multiple strings)
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); // s == "tic-tac-toe"
    // format! does NOT take ownership

    // 4. Indexing & Slicing Strings (and why you can't index Strings)
    // ---------------------------------------------------------------
    // - Strings are Vec<u8> (bytes), not Vec<char>.
    // - Each Unicode scalar may take multiple bytes.
    // - Direct indexing (e.g. s[0]) is not allowed, because it may not be a valid character boundary.

    // Example:
    // let s1 = String::from("hi");
    // let h = s1[0]; // ERROR: strings cannot be indexed

    // - Length in bytes != number of characters
    let hello = String::from("Здравствуйте");
    println!("Length = {}", hello.len()); // 24 bytes, not 12 chars!

    // - String slices must be on char boundaries, otherwise program panics:
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // valid, s == "Зд"
    // let bad = &hello[0..1]; // panics at runtime!

    // 5. Iterating over Strings
    // -------------------------
    // - .chars(): iterate over Unicode scalar values (char)
    for c in "Зд".chars() {
        println!("{c}");
    }
    // Output: З д

    // - .bytes(): iterate over raw bytes
    for b in "Зд".bytes() {
        println!("{b}");
    }
    // Output: 208 151 208 180 (raw byte values)

    // - Grapheme clusters (user-perceived "letters") are NOT in std; use external crates like unicode-segmentation.

    // 6. Summary
    // ----------
    // - Strings are complex due to UTF-8 and Unicode.
    // - Rust exposes this complexity to prevent bugs and invalid memory access.
    // - Use .chars(), .bytes(), and string slices carefully.
    // - Many useful methods: contains, replace, etc.

    println!("See source for notes and examples on UTF-8 Strings in Rust.");
}