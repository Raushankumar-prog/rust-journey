// Chapter 9.1: Unrecoverable Errors with panic! - Notes & Examples

pub fn c9_1() {
    // 1. What is panic!?
    // ------------------
    // - Rust's way of handling unrecoverable errors.
    // - Causes the program to print a failure message, clean up the stack, and quit.
    // - Two ways to panic:
    //   a) Code triggers a panic (e.g., out-of-bounds access)
    //   b) Explicit call to panic! macro

    // 2. Unwinding vs. Aborting
    // -------------------------
    // - By default: panic! unwinds the stack, cleaning up each function.
    // - You can switch to aborting (immediate exit, no cleanup) for smaller binaries.
    //   Set in Cargo.toml:
    //   [profile.release]
    //   panic = 'abort'

    // 3. Example: Explicit panic!
    // ---------------------------
    // This code will panic and print a message:
    // panic!("crash and burn");

    // 4. Example: Panic from a library (out-of-bounds)
    let v = vec![1, 2, 3];
    // v[99]; // Panics: index out of bounds

    // 5. Interpreting Panic Output & Backtraces
    // -----------------------------------------
    // - Panic output shows message and code location (file:line:column).
    // - For more detail, set environment variable: RUST_BACKTRACE=1
    // - Backtrace shows call stack: read from top down to find your code, then work up to see what led to the panic.

    // 6. Why Panic on Out-of-Bounds?
    // ------------------------------
    // - Prevents undefined behavior and security vulnerabilities
    //   (e.g., buffer overreads in C)
    // - Rust stops execution to ensure safety

    // 7. Fixing Panics
    // ----------------
    // - Investigate the line(s) in your code where panic occurred.
    // - Adjust logic so invalid operations (e.g. out-of-range access) don't happen.

    // 8. When to Use panic!
    // ---------------------
    // - Only in situations where your code cannot recover (program invariants broken, bugs, etc.).
    // - For recoverable errors, use Result (see next section).

    println!("See source for notes and examples on unrecoverable errors and panic! in Rust.");
}