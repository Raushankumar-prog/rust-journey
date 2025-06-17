// Chapter 3 Notes: Variables, Data Types, Functions, Control Flow

// -- Variables and Mutability --
// - Variables are immutable by default: `let x = 5;`
// - Trying to change `x` (e.g., `x = 6;`) gives a compiler error.
// - To make a variable mutable, use `let mut x = 5;`.
// - Mutability is explicit, helping code safety and clarity.

// -- Constants --
// - Constants use `const` keyword and must have a type: `const MAX_POINTS: u32 = 100_000;`
// - Always immutable, can't use `mut`.
// - Must be set to a constant expression, not runtime value.
// - Naming: SCREAMING_SNAKE_CASE.

// -- Shadowing --
// - Can declare a new variable with same name: `let x = ...; let x = x + 1;`
// - Shadowing allows changing type and value, unlike `mut`.
// - Example: `let spaces = "   "; let spaces = spaces.len();`
// - Using `mut` does not allow type change.

// -- Data Types --
// - Rust is statically typed; types mostly inferred but sometimes must be annotated.

// --- Scalar Types ---
// 1. Integers: Signed (i8, i16, i32, i64, i128, isize), Unsigned (u8, ...)
//    - Default is i32.
//    - Literal forms: decimal, hex (0xff), octal (0o77), binary (0b1010), byte (b'A').
//    - Integer overflow: panics in debug, wraps in release (`wrapping_*`, `checked_*`, etc.).
// 2. Floating-Point: f32, f64 (default is f64).
// 3. Boolean: `bool` (`true`, `false`).
// 4. Character: `char` (single quotes, Unicode scalar values, 4 bytes).

// --- Compound Types ---
// 1. Tuple: Fixed length, possibly mixed types. Example: `(i32, f64, u8)`.
//    - Destructure: `let (x, y, z) = tup;`
//    - Access: `tup.0`
//    - Unit type: `()` (empty tuple).
// 2. Array: `[i32; 5] = [1, 2, 3, 4, 5];`
//    - Fixed size, all elements same type.
//    - Useful for stack-allocated, fixed-size data.
//    - Initialize all elements: `let a = [3; 5];`
//    - Access: `a[0]`
//    - Out-of-bounds access panics at runtime.

// -- Functions --
// - Declare with `fn` keyword; main entry is `fn main() {}`.
// - Snake case naming convention.
// - Function parameters must have types: `fn foo(x: i32)`.
// - Multiple parameters: comma-separated.
// - Function bodies: series of statements, optionally ending with an expression (no semicolon).

// -- Statements vs. Expressions --
// - Statement: does not return a value, e.g., `let x = 6;`
// - Expression: evaluates to a value, e.g., `x + 1`
// - Blocks `{ ... }` are expressions; last line (w/o `;`) is value.

// -- Return Values --
// - Specify with arrow: `fn five() -> i32 { 5 }`
// - Last expression is returned automatically; `return` is optional but can be used.
// - Adding a semicolon turns expression into a statement (returns `()`).

// -- Control Flow --

// --- if Expressions ---
// - Branch execution: `if condition { ... } else { ... }`
// - Condition must be `bool`.
// - `else if` for multiple branches.
// - `if` is an expression; can assign: `let x = if cond { 1 } else { 2 };`
// - Both arms must return same type.

// --- Loops ---
// 1. loop: Infinite, break with `break;` (can return value: `break val;`)
//    - Can label loops for breaking out of outer loops: `'outer: loop { ... break 'outer; }`
// 2. while: Loop while a condition is true.
// 3. for: Iterate over a collection (array, range).
//    - Safer and more concise than manual indexing.
//    - Example: `for el in arr { ... }`
//    - Can use ranges: `for i in (1..4).rev() { ... }`

// -- Summary Practice Ideas --
// - Temperature converter
// - Fibonacci number generator
// - Print "The Twelve Days of Christmas" song using loops

// -- Next Chapter --
// Ownership (a unique Rust concept)


// --- Variable Mutability and Shadowing ---
fn variable_examples() {
    // Immutable by default
    let x = 5;
    println!("Immutable x: {x}");

    // Mutable variable
    let mut y = 10;
    println!("Mutable y before: {y}");
    y = 15;
    println!("Mutable y after: {y}");

    // Constant
    const SECONDS_IN_AN_HOUR: u32 = 60 * 60;
    println!("Seconds in an hour: {SECONDS_IN_AN_HOUR}");

    // Shadowing (can change type)
    let z = "hello";
    let z = z.len(); // z is now usize, not &str
    println!("Shadowed z (length): {z}");
}

// --- Data Types: Scalars and Compound Types ---
fn scalar_and_compound_types() {
    // Scalar types
    let int: i32 = -42;
    let float = 3.1415; // f64 by default
    let is_true: bool = true;
    let letter: char = 'R';

    println!("int = {int}, float = {float}, is_true = {is_true}, letter = {letter}");

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("Tuple destructure: a={a}, b={b}, c={c}");
    println!("Access tuple directly: tup.0 = {}, tup.1 = {}", tup.0, tup.1);

    // Array
    let arr = [1, 2, 3, 4, 5];
    println!("Array first element: {}", arr[0]);

    let same_value_arr = [3; 4]; // [3, 3, 3, 3]
    println!("Array with same values: {:?}", same_value_arr);
}

// --- Functions and Return Values ---
fn add(x: i32, y: i32) -> i32 {
    // No semicolon: returns x + y
    x + y
}

fn function_examples() {
    fn print_measurement(value: i32, unit: char) {
        println!("Measurement: {value}{unit}");
    }
    print_measurement(42, 'm');

    let sum = add(7, 8);
    println!("Sum from add(): {sum}");

    // Expression block
    let expr_result = {
        let temp = 2;
        temp * temp + 1
    };
    println!("Result from expression block: {expr_result}");
}

// --- Control Flow: if, loops, while, for ---
fn control_flow_examples() {
    // if-else
    let n = 7;
    if n < 5 {
        println!("{n} is less than 5");
    } else if n == 5 {
        println!("{n} is exactly 5");
    } else {
        println!("{n} is greater than 5");
    }

    // if as expression
    let parity = if n % 2 == 0 { "even" } else { "odd" };
    println!("{n} is {parity}");

    // loop with break and value
    let mut count = 0;
    let double = loop {
        count += 1;
        if count == 4 {
            break count * 2;
        }
    };
    println!("Loop returned: {double}");

    // while loop
    let mut countdown = 3;
    while countdown > 0 {
        println!("{countdown}...");
        countdown -= 1;
    }
    println!("LIFTOFF!");

    // for loop over array
    let nums = [10, 20, 30, 40];
    for val in nums {
        println!("Array element: {val}");
    }

    // for loop with range and rev
    for i in (1..=3).rev() {
        println!("{i}!");
    }
    println!("Go!");
}

// The following function demonstrates all of the above examples.
// Remove or comment out the following when integrating parts above into a real project.
pub fn ch3() {
    println!("--- Variable Examples ---");
    variable_examples();

    println!("\n--- Scalar and Compound Types ---");
    scalar_and_compound_types();

    println!("\n--- Function Examples ---");
    function_examples();

    println!("\n--- Control Flow Examples ---");
    control_flow_examples();
}

