// Chapter 11: How to Write Tests in Rust
// --------------------------------------
// This file contains summary notes and example code for Rust testing concepts.
// Each section is commented with explanations and example code.

/// ch11: Summary and Examples of Rust Testing Features
///
/// 1. What is a Test in Rust?
///    - Test functions verify your code behaves as expected.
///    - Mark tests with `#[test]`, run them with `cargo test`.
///
/// 2. Anatomy of a Test Function
///    - Use `#[test]` above the function.
///    - Typical actions: setup, run code, assert result.
///
/// 3. Test Module Structure
///    - Use `#[cfg(test)]` to only compile test code for `cargo test`.
///    - Define tests in a `mod tests` block, use `use super::*;` for access.
///
/// 4. Assertions
///    - `assert!(cond)`: Condition is true.
///    - `assert_eq!(a, b)`: a == b.
///    - `assert_ne!(a, b)`: a != b.
///    - Add custom messages for failures.
///
/// 5. Panics in Tests
///    - Use `#[should_panic]` to check for panics.
///    - Use `#[should_panic(expected = "...")]` to match panic messages.
///
/// 6. Result<T, E> in Tests
///    - Test can return `Result<(), String>`.
///    - Use `?` for error propagation.
///    - Return `Ok(())` to pass, `Err(...)` to fail.
///
/// 7. Practical Tips
///    - Group tests, use custom messages, filter with `cargo test`.
///    - Tests run in parallel, each in its own thread.

/// Example 1: A simple function to test
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Example 2: A struct and method for assertion example
#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/// Example 3: A function that can panic
pub struct Guess {
    pub value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        Guess { value }
    }
}

/// Example 4: A function for Result-based testing
pub fn add_two(a: usize) -> usize {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    // Basic test with assert_eq!
    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // Test with assert! macro for boolean condition
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        assert!(larger.can_hold(&smaller));
    }

    // Test with assert! macro for the opposite case
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        assert!(!smaller.can_hold(&larger));
    }

    // Test that expects a panic
    #[test]
    #[should_panic]
    fn guess_greater_than_100_should_panic() {
        Guess::new(200);
    }

    // Test that expects a specific panic message
    #[test]
    #[should_panic(expected = "between 1 and 100")]
    fn guess_less_than_1_should_panic() {
        Guess::new(0);
    }

    // Test with custom failure message
    #[test]
    fn greeting_contains_name() {
        let name = "Carol";
        let result = format!("Hello {name}!");
        assert!(
            result.contains(name),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    // Test returning Result<(), String>
    #[test]
    fn test_add_two_result() -> Result<(), String> {
        let result = add_two(2);
        if result == 4 {
            Ok(())
        } else {
            Err(format!("Expected 4, got {}", result))
        }
    }
}