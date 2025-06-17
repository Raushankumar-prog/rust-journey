// Chapter 11: Test Organization in Rust
// -------------------------------------
// Summary notes and code snippets for understanding Unit and Integration Tests.

pub fn ch11_3() {
    /*
    ----------------------------------------------------------------------
    Test Organization in Rust: Unit Tests vs Integration Tests
    ----------------------------------------------------------------------

    1. Two Main Categories:
       - Unit Tests: Small, focused, can access private code.
       - Integration Tests: External, test the public API, simulate real-world use.

    ----------------------------------------------------------------------
    2. Unit Tests
       - Test code in isolation, usually per module or function.
       - Placed in the same file as the code (src/).
       - Convention: create a module named `tests` at the bottom of each file.
       - Use #[cfg(test)] to only compile/run tests with `cargo test`.
       - Can test private/non-pub items because of module scoping.
       
       Example (src/lib.rs):
           pub fn add(left: u64, right: u64) -> u64 {
               left + right
           }
           
           #[cfg(test)]
           mod tests {
               use super::*;
               #[test]
               fn it_works() {
                   let result = add(2, 2);
                   assert_eq!(result, 4);
               }
           }

    ----------------------------------------------------------------------
    3. Testing Private Functions
       - Unit tests can directly call private functions in their own module/file.
       
       Example:
           pub fn add_two(a: usize) -> usize {
               internal_adder(a, 2)
           }
           fn internal_adder(left: usize, right: usize) -> usize {
               left + right
           }
           #[cfg(test)]
           mod tests {
               use super::*;
               #[test]
               fn internal() {
                   let result = internal_adder(2, 2);
                   assert_eq!(result, 4);
               }
           }

    ----------------------------------------------------------------------
    4. Integration Tests
       - Placed in the top-level `tests/` directory (not in src/).
       - Each file is a separate crate.
       - Only access the library's public API (cannot access private items).
       - No need for #[cfg(test)].
       
       Example (tests/integration_test.rs):
           use adder::add_two;
           #[test]
           fn it_adds_two() {
               let result = add_two(2);
               assert_eq!(result, 4);
           }

    ----------------------------------------------------------------------
    5. How Integration Tests Work
       - `cargo test` compiles and runs:
         - Unit tests (src/)
         - Integration tests (tests/)
         - Doc tests (in rustdoc comments)
       - If any unit test fails, integration and doc tests are skipped.

    ----------------------------------------------------------------------
    6. Organizing Integration Tests and Shared Helpers
       - Each file in tests/ is a separate crate, files can't share helpers directly.
       - To share code, use a subdirectory: tests/common/mod.rs
       - In each integration test file, declare: mod common;
       - Do not use tests/common.rs (or it will be treated as a test file itself and show up as "running 0 tests").
       
       Example structure:
           tests/
               common/
                   mod.rs
               integration_test.rs

    ----------------------------------------------------------------------
    7. Integration Tests for Binary Crates
       - Only library crates can be used in integration tests via `use`.
       - If you want to test binary logic, move logic from main.rs to lib.rs, and call it from main.

    ----------------------------------------------------------------------
    8. Summary Table

        | Test Type       | Location         | Test Private? | Needs #[cfg(test)]? | How to Import Code      |
        |-----------------|------------------|---------------|---------------------|-------------------------|
        | Unit Test       | src/ files       | Yes           | Yes                 | use super::*;           |
        | Integration     | tests/ directory | No            | No                  | use mycrate::function;  |

    ----------------------------------------------------------------------
    9. Key Points
       - Use unit tests for internals and private code.
       - Use integration tests for end-to-end/public API testing.
       - #[cfg(test)] only needed for unit tests.
       - Place shared integration test helpers in tests/common/mod.rs.
       - For binary crates: put logic in lib.rs for testability.

    */
}