// Chapter 10.3: Validating References with Lifetimes - Notes & Examples

pub fn ch10_3() {
    // 1. What are Lifetimes?
    // ----------------------
    // - Lifetimes are generics that ensure references are valid as long as needed.
    // - Prevent dangling references (invalid memory access).

    // 2. Dangling Reference Example (won't compile)
    // ---------------------------------------------
    // let r;
    // {
    //     let x = 5;
    //     r = &x; // x goes out of scope here!
    // }
    // println!("r: {r}"); // Error: r refers to invalid memory

    // 3. Borrow Checker ensures lifetimes are valid.
    // ----------------------------------------------
    // Rust checks that references do not outlive the data they reference.

    // 4. Lifetime Annotations in Functions
    // ------------------------------------
    // Function returning the longer of two string slices requires explicit lifetimes:
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }
    // 'a ties the lifetime of the returned reference to the lifetimes of the inputs.

    // 5. Lifetime Annotations in Structs
    // ----------------------------------
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    // 6. Lifetime Elision Rules
    // ------------------------
    // - Rust infers lifetimes in simple cases (e.g. fn first_word(s: &str) -> &str).
    // - If there's one input lifetime, it's assigned to all output lifetimes.
    // - If method, &self lifetime is assigned to output.

    // 7. Lifetime in Methods
    // ----------------------
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 { 3 }
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {announcement}");
            self.part
        }
    }

    // 8. The 'static Lifetime
    // -----------------------
    let s: &'static str = "I have a static lifetime.";
    // All string literals are 'static (live for the duration of the program).

    // 9. Combining Lifetimes, Traits, and Generics
    // --------------------------------------------
    use std::fmt::Display;
    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {ann}");
        if x.len() > y.len() { x } else { y }
    }

    // 10. Summary
    // -----------
    // - Lifetimes ensure references are always valid.
    // - Use annotations when lifetimes of parameters/returns are related.
    // - Most of the time, the compiler infers lifetimes. Annotate when ambiguous.
    // - Lifetimes, generics, and traits let you write flexible, safe, reusable code.

    println!("See source for notes and examples on validating references with lifetimes in Rust.");
}