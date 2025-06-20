// Chapter 9.3: To panic! or Not to panic! - Notes & Guidelines

pub fn ch9_3() {
    // 1. When to panic! vs. when to return Result
    // -------------------------------------------
    // - panic! is for unrecoverable errors; you give no option for recovery.
    // - Result gives the caller a choice to handle or propagate the error.
    // - Default to Result for functions that might fail; use panic! for truly exceptional cases.

    // 2. Appropriate use cases for panic!
    // -----------------------------------
    // - Example/prototype/test code (unwrap/expect is fine for brevity and clarity).
    // - When logic guarantees Ok, but compiler can't verify it (e.g. hardcoded valid inputs):
    use std::net::IpAddr;
    let home: IpAddr = "127.0.0.1".parse().expect("Hardcoded IP address should be valid");

    // 3. Guidelines for panic!
    // ------------------------
    // - Panic when invariants, contracts, or guarantees are broken (invalid/contradictory/missing data).
    // - Panic when continuing would be insecure or harmful.
    // - Panic if function contract is violated (e.g. out-of-bounds access).
    // - Panic for programmer/caller errors, not for user errors or expected failures.

    // 4. Prefer Result for expected failures
    // --------------------------------------
    // - Use Result when failure is a normal, expected outcome (e.g. parse errors, HTTP errors).
    // - Returning Result lets the caller decide what to do.

    // 5. Use the type system for validation
    // -------------------------------------
    // - Prefer encoding invariants in types (e.g. unsigned types, custom structs) to avoid runtime checks.
    // - Example: Use a custom type to guarantee a value is in a valid range.
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {value}.");
            }
            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

    // 6. Summary
    // ----------
    // - Use panic! for unrecoverable errors, contract violations, and bugs.
    // - Use Result for recoverable/expected errors.
    // - Use the type system to prevent invalid states.
    // - Document panics in your API.

    println!("See source for guidelines and examples on panic! vs Result error-handling choices in Rust.");
}