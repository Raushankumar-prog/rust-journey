// =================== Structs in Rust ===================

// Structs are similar to tuples but with named fields.
// This makes them more flexible and readable than tuples.

/// Define a struct with named fields.
struct User {
    active: bool,
    username: String,  // owned types preferred
    email: String,
    sign_in_count: u64,
}

pub fn ch5_1() {
    // Create a new instance of User
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Accessing a field using dot notation
    println!("Email: {}", user1.email);

    // Modifying a field (entire struct must be mutable)
    user1.email = String::from("anotheremail@example.com");

    // ======================================================
    // Field Init Shorthand: When parameter names == field names
    // ======================================================

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username, // shorthand
            email,    // shorthand
            sign_in_count: 1,
        }
    }

    let user2 = build_user(
        String::from("test@example.com"),
        String::from("testuser"),
    );

    // ======================================================
    // Struct Update Syntax: Copy remaining fields from another instance
    // ======================================================

    // Using ..user1 moves values, so user1.username is no longer valid after this
    let user3 = User {
        email: String::from("user3@example.com"),
        ..user2
    };

    // user2.username has been moved into user3 (String is not Copy)

    // ======================================================
    // Tuple Structs: Useful when field names are unnecessary
    // ======================================================

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Different types even though fields are same
    // let c: Color = origin; // ❌ Error

    // Destructuring tuple structs
    let Point(x, y, z) = origin;

    // ======================================================
    // Unit-Like Structs: Structs with no fields
    // Useful for trait implementation without storing data
    // ======================================================

    struct AlwaysEqual;

    let _instance = AlwaysEqual;

    // ======================================================
    // Ownership of Struct Fields
    // ======================================================
    // Use owned types (like String) instead of references (&str) to avoid lifetime issues.

    // ❌ This will NOT compile due to missing lifetimes:
    /*
    struct UserRef {
        active: bool,
        username: &str,   // ❌
        email: &str,      // ❌
        sign_in_count: u64,
    }
    */

    // ✅ Correct version would be:
    /*
    struct UserRef<'a> {
        active: bool,
        username: &'a str,
        email: &'a str,
        sign_in_count: u64,
    }
    */

    // Lifetimes tell Rust how long references are valid. (Covered in Chapter 10)
}
