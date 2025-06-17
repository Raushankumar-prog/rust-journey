// Chapter 11, part 2: Controlling How Tests Are Run
// -------------------------------------------------
// This function contains commented notes and code examples
// to help understand test control in Rust (parallelism, output, filtering, ignoring).

pub fn ch11_2() {
    // 1. Running tests in parallel (default behavior)
    // - Rust runs tests in parallel using threads for speed.
    // - Don't let tests interfere via shared state (files, env vars, etc).
    //
    // Example: If two tests write to the same file, they may clash.

    // 2. Run tests sequentially (one at a time)
    // - Use the --test-threads=1 flag after '--' to run tests one by one:
    // $ cargo test -- --test-threads=1
    //
    // This avoids interference, but is slower.

    // 3. Capturing Output
    // - By default, test output (e.g., println!) is hidden for passing tests.
    // - Output is shown for failed tests.
    // - To always show output, use:
    // $ cargo test -- --show-output

    // Example function and tests:
    fn prints_and_returns_10(a: i32) -> i32 {
        println!("I got the value {a}");
        10
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn this_test_will_pass() {
            let value = prints_and_returns_10(4);
            assert_eq!(value, 10); // Output is hidden unless --show-output is used
        }

        #[test]
        fn this_test_will_fail() {
            let value = prints_and_returns_10(8);
            assert_eq!(value, 5); // Output will be shown because test fails
        }
    }

    // 4. Running a subset of tests by name
    // - You can run only tests whose name matches a string:
    // $ cargo test one_hundred
    // - Or run those whose name contains part of a string:
    // $ cargo test add

    // 5. Ignoring time-consuming tests
    // - Use #[ignore] to mark a test as ignored by default:
    // #[test]
    // #[ignore]
    // fn expensive_test() { ... }
    //
    // - Run only ignored tests:
    // $ cargo test -- --ignored
    // - Run all tests, including ignored:
    // $ cargo test -- --include-ignored

    // 6. Summary table for controlling tests:
    // | What You Want To Do                   | Command                                     |
    // |----------------------------------------|---------------------------------------------|
    // | Run all tests (default)                | cargo test                                  |
    // | Run tests with specific name           | cargo test part_of_name                     |
    // | Run tests sequentially                 | cargo test -- --test-threads=1              |
    // | Show all output (even on success)      | cargo test -- --show-output                 |
    // | Run ignored tests only                 | cargo test -- --ignored                     |
    // | Run all tests, including ignored ones  | cargo test -- --include-ignored             |

    // Note: All flags after '--' are passed to the test binary, not to cargo itself.
}