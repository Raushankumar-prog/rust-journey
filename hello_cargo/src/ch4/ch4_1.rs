// Chapter 4.1 Notes: What Is Ownership? (Rust Book)

// -- Ownership Overview --
// - Ownership is Rust's mechanism for memory management without GC.
// - Compiler enforces ownership rules at compile time; violations cause errors, not runtime bugs.
// - Key goals: safety, no runtime overhead, no double-frees, no leaks, no use-after-free.

// -- Stack vs Heap --
// - Stack: stores values in order (LIFO), fast, fixed size at compile time.
// - Heap: for dynamically sized or unknown-at-compile-time data, slower to allocate/access, requires tracking allocation and deallocation.
// - Stack data is trivially copied; heap data needs explicit management (often via pointers).

// -- Ownership Rules --
// 1. Each value in Rust has a single owner.
// 2. Only one owner at a time.
// 3. When the owner goes out of scope, the value is dropped (memory freed).

// -- Variable Scope --
// - A scope is the region of code for which a variable is valid.
// - When a variable goes out of scope, Rust calls `drop` to clean up resources (RAII).

// Example:
// {
//   let s = "hello"; // valid in this scope
// } // s is dropped here

// -- The String Type --
// - String literals are hardcoded, immutable, and stored in the binary.
// - String type (`String::from("hello")`) is heap-allocated, growable, and mutable.
// - Heap-allocated memory must be returned to allocator when done (Rust does this automatically).

// Example (String mutation):
// let mut s = String::from("hello");
// s.push_str(", world!"); // now s contains "hello, world!"

// -- Move Semantics --
// - Assignment of stack-only data (like integers) simply copies the value (both variables valid).
// - Assignment of heap data (like String) moves the ownership; the original variable becomes invalid.
// - Prevents double frees and memory bugs.

// let x = 5;
// let y = x; // both x and y are valid

// let s1 = String::from("hello");
// let s2 = s1; // s1 is moved into s2; s1 is invalid after this

// println!("{s1}"); // error: borrow of moved value

// - This is called a "move", not a shallow copy; deep copying is explicit and expensive.

// -- Assignment and Drop --
// - Assigning a new heap value to an existing variable drops the old value immediately.
// let mut s = String::from("hello");
// s = String::from("ahoy"); // "hello" is dropped, s now owns "ahoy"

// -- Cloning Heap Data --
// - Use `clone()` for deep copy (heap data duplicated):
// let s1 = String::from("hello");
// let s2 = s1.clone(); // both s1 and s2 now own independent heap data

// -- Stack-only Data: Copy Trait --
// - Simple types (integers, bool, char, floats, tuples of Copy types) implement Copy trait.
// - Assignment or passing to functions copies these values, both remain valid.
// - Types with heap data or special drop logic do NOT implement Copy.

// -- Ownership and Functions --
// - Passing ownership to a function moves it (unless type is Copy).
// - After passing, the original variable is invalid (unless Copy).
// - Return values can transfer ownership back to the caller.

// fn takes_ownership(s: String) { ... } // moves ownership in
// fn makes_copy(x: i32) { ... } // i32 is Copy, so x still valid after

// - Returning values can move ownership back out:

// fn gives_ownership() -> String { String::from("yours") }
// fn takes_and_gives_back(a: String) -> String { a }

// -- Tedious Ownership Transfers --
// - To keep using a value after passing to a function, you must return it (often bundled in a tuple).
// - Example:
// fn calculate_length(s: String) -> (String, usize) {
//    let len = s.len();
//    (s, len)
// }

// let (s2, len) = calculate_length(s1);

// - This is clunky for common use cases. Rust provides references to solve this (next section).

// ---
// Summary: Ownership lets Rust manage memory automatically and safely, but requires understanding move semantics. Heap data moves by default; copying is explicit; stack data is trivially Copy. Functions move or copy ownership by default. References (next) allow borrowing without moving ownership.


// ---- Example code for Chapter 4.1 Ownership ----
pub fn ch4_1_demo() {
    // Stack data: Copy
    let x = 42;
    let y = x;
    println!("x = {x}, y = {y}"); // Both valid

    // Heap data: Move
    let s1 = String::from("hello");
    let s2 = s1; // s1 moved to s2
    // println!("{s1}"); // Uncommenting this line would cause a compile error!
    println!("s2 = {s2}");

    // Clone for deep copy
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3 = {s3}, s4 = {s4}");

    // Ownership and functions
    let s5 = String::from("Rust");
    takes_ownership(s5); // s5 is moved here and dropped at end of function
    // println!("{s5}"); // Compile error: s5 was moved

    let n = 100;
    makes_copy(n); // n implements Copy, so still valid
    println!("n = {n}");

    // Return ownership
    let s6 = gives_ownership();
    println!("s6 = {s6}");
    let s7 = String::from("ownership");
    let s8 = takes_and_gives_back(s7);
    println!("s8 = {s8}");

    // Tuple trick for getting value and its property without references
    let s9 = String::from("tuple");
    let (s10, len) = calculate_length(s9);
    println!("The length of '{s10}' is {len}.");
}

fn takes_ownership(some_string: String) {
    println!("Took ownership of: {some_string}");
}

fn makes_copy(some_int: i32) {
    println!("Copied int: {some_int}");
}

fn gives_ownership() -> String {
    String::from("gift")
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

