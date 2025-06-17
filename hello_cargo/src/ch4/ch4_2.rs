// Chapter 4.2 Notes: References and Borrowing (Rust Book)

// -- Motivation: Why References? --
// - Ownership rules mean you can't use a value after passing it to a function (unless you return it).
// - Returning values and ownership everywhere is tedious and clutters APIs.
// - References allow borrowing a value without taking ownership.

// -- References --
// - Reference: &T (read-only), &mut T (mutable), pronounced "a reference to T"
// - Syntax: `&s1` passes a reference to `s1`.
// - Function signature: `fn foo(x: &Type)` means "x is a reference to Type".
// - A reference lets you access the value it points to, but does not own the value.
// - When the reference goes out of scope, the value is NOT dropped.

// Example: calculate_length with reference (read-only)
// fn calculate_length(s: &String) -> usize { s.len() }
// let s1 = String::from("hello");
// let len = calculate_length(&s1);
// println!("The length of '{s1}' is {len}.");
// - s1 is still valid after the function call!

// -- Borrowing --
// - Creating a reference is called "borrowing".
// - The owner keeps ownership; the borrower just has access for a while.

// -- Mutability and References --
// - References are immutable by default, just like variables.
// - You cannot modify through an immutable reference.
// - To allow modification, you must make a mutable reference: &mut T

// Example: (does NOT compile!)
// fn change(some_string: &String) { some_string.push_str(", world"); }
// let mut s = String::from("hello");
// change(&s); // error: cannot borrow as mutable

// -- Mutable References --
// - To allow a function to modify a value, use &mut:
// fn change(some_string: &mut String) { some_string.push_str(", world"); }
// let mut s = String::from("hello");
// change(&mut s); // OK, s is now "hello, world"

// --- Rules for References ---
// 1. At any given time, you can have either
//      a) one mutable reference
//      b) or any number of immutable references
//    (not both at the same time!)
// 2. References must always be valid (no dangling pointers).

// - The restriction prevents data races (compile time!)
// - Data race: two or more pointers access same data at same time, at least one is writing, and no sync mechanism.

// Example: (does NOT compile!)
// let mut s = String::from("hello");
// let r1 = &mut s;
// let r2 = &mut s; // error: cannot borrow s as mutable more than once

// - Multiple immutable references are allowed:
// let r1 = &s; let r2 = &s; // OK

// - You cannot have a mutable reference while immutable references are in use:
// let r1 = &s;
// let r2 = &mut s; // error!

// - Rust allows you to create a new mutable reference after all immutable references go out of use (scope ends).

// -- Reference Lifetimes --
// - The scope of a reference starts where it's introduced and ends at its last use.
// - You can reuse mutable or immutable references in separate, non-overlapping scopes.

// Example (OK):
// let mut s = String::from("hello");
// let r1 = &s;
// println!("{r1}");
// let r2 = &mut s;
// println!("{r2}");

// -- Dangling References --
// - Rust prevents returning a reference to a value that will be dropped (dangling pointer).
// - Example (does NOT compile!):
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// } // s goes out of scope, &s would point to freed memory

// - Solution: return the owned value instead:
// fn no_dangle() -> String { String::from("hello") }

// -- Summary of Rules --
// 1. You can have either one mutable reference or any number of immutable references to a value, not both at once.
// 2. References must always point to valid data (no dangling refs).

// Next: Slices (next section in Rust book)


// ---- Example code for Chapter 4.2 References and Borrowing ----

pub fn ch4_2_demo() {
    // Immutable reference (borrowing)
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    // Mutable reference
    let mut s2 = String::from("hi");
    change(&mut s2);
    println!("s2 after mutation: {s2}");

    // Multiple immutable references: OK
    let s3 = String::from("immut");
    let r1 = &s3;
    let r2 = &s3;
    println!("r1 = {r1}, r2 = {r2}");

    // Multiple mutable references: NOT OK
    // let mut s4 = String::from("nope");
    // let m1 = &mut s4;
    // let m2 = &mut s4; // Error

    // Mixing mutable and immutable references: NOT OK
    // let mut s5 = String::from("mix");
    // let i1 = &s5;
    // let m1 = &mut s5; // Error

    // Dangling reference: NOT OK
    // let d = dangle(); // Error

    // Non-overlapping references: OK
    let mut s6 = String::from("scopes");
    let i1 = &s6;
    println!("i1 = {i1}");
    let m1 = &mut s6;
    m1.push_str("!");
    println!("m1 = {m1}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

// fn dangle() -> &String {
//     let s = String::from("dangling");
//     &s // Error: returns reference to dropped value
// }

