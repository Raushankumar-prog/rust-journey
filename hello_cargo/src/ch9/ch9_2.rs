// Chapter 9.2: Recoverable Errors with Result - Notes & Examples

pub fn ch9_2() {
    // 1. What is Result?
    // ------------------
    // - Enum for recoverable errors.
    //   enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    //   }
    // - T: success value type, E: error value type

    // 2. Example: Opening a File with Result
    // --------------------------------------
    use std::fs::File;
    let greeting_file_result = File::open("hello.txt");

    // 3. Handling Result with match
    // -----------------------------
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };

    // 4. Matching on Specific Errors
    // ------------------------------
    use std::io::ErrorKind;
    let greeting_file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => panic!("Problem opening the file: {error:?}"),
        },
    };

    // 5. Alternatives: Closures and Helper Methods
    // --------------------------------------------
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    // 6. Shortcuts: unwrap & expect
    // -----------------------------
    // .unwrap() => Ok(T) returns T, Err(E) panics.
    // .expect("msg") => Ok(T) returns T, Err(E) panics with "msg".
    let greeting_file = File::open("hello.txt").unwrap();
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

    // 7. Propagating Errors (returning Result)
    // ----------------------------------------
    use std::io::{self, Read};
    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    // 8. The ? Operator
    // -----------------
    // - Returns early if Err, otherwise unwraps Ok.
    // - Chaining:
    fn read_username_from_file_short() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }

    // Even shorter using std::fs::read_to_string:
    use std::fs;
    fn read_username_from_file_shortest() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    // 9. ? Operator with Option
    // -------------------------
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    // 10. Using ? in main()
    // ---------------------
    // - main can return Result<(), E> for ? operator use.
    use std::error::Error;
    fn main() -> Result<(), Box<dyn Error>> {
        let greeting_file = File::open("hello.txt")?;
        Ok(())
    }

    println!("See source for notes and examples on recoverable errors and Result in Rust.");
}