

pub fn ch4_3_demo() {
    // --- String Slices ---

    // A String is stored on the heap and can be mutated
    let s = String::from("hello world");

    // String slices are references to a part of a String.
    // They do NOT own the data.
    let hello = &s[0..5];  // slice of "hello"
    let world = &s[6..11]; // slice of "world"

    println!("hello: {}, world: {}", hello, world);

    // You can omit 0 at the beginning or len at the end
    let slice1 = &s[..5];  // same as &s[0..5]
    let slice2 = &s[6..];  // same as &s[6..11]
    let slice3 = &s[..];   // whole string

    println!("slice1: {}, slice2: {}, slice3: {}", slice1, slice2, slice3);

    // --- first_word function using string slices ---
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    let word = first_word(&s);
    println!("first word: {}", word);

    // --- Demonstrating compiler error with mutable and immutable borrow ---
    // The below code will NOT compile if uncommented.
    /*
    let mut s2 = String::from("hello world");
    let w = first_word(&s2);
    s2.clear(); // Error: cannot borrow `s2` as mutable because it's already borrowed as immutable
    println!("w: {}", w);
    */

    // --- String Literals are slices ---

    let literal = "Hello, Rust!";
    let slice_from_literal = &literal[0..5]; // slice of a &str
    println!("slice from literal: {}", slice_from_literal);

    // --- Slices with Arrays ---

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..4]; // slice contains [2, 3, 4]
    println!("slice from array: {:?}", slice);

    // Slices prevent bugs by ensuring references stay valid and synchronized
    // Rust's borrow checker guarantees that slices can’t outlive the data they point to
}
