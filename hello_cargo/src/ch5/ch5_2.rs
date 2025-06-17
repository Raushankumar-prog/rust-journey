// Chapter 5.2 Notes: Example Program Using Structs (Rust Book)

// -- Motivation: Why Structs? --
// - Grouping related data (like width and height for a rectangle) is clearer and less error-prone than using separate variables or tuples.
// - Tuples give structure but not meaning; struct fields have names, improving readability and safety.

// -- Step 1: Basic Area Calculation (separate variables)
// fn main() {
//     let width1 = 30;
//     let height1 = 50;
//     println!("The area of the rectangle is {} square pixels.", area(width1, height1));
// }
// fn area(width: u32, height: u32) -> u32 { width * height }

// -- Step 2: Tuple Version
// fn main() {
//     let rect1 = (30, 50);
//     println!("The area of the rectangle is {} square pixels.", area(rect1));
// }
// fn area(dimensions: (u32, u32)) -> u32 { dimensions.0 * dimensions.1 }
// - Drawback: indices (0, 1) are not meaningful; easy to make mistakes.

// -- Step 3: Struct Version
// - Structs give names to the whole (Rectangle) and to each part (width, height).
// - Improves code clarity and maintainability.

/// Rectangle struct definition
struct Rectangle {
    width: u32,
    height: u32,
}

fn ch5_2_basic_struct_demo() {
    // Create a Rectangle instance
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

/// Calculate area by borrowing a Rectangle struct.
/// Borrowing (&Rectangle) allows us to use rect1 after calling area().
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// -- Debug Printing Structs --
// - By default, structs do not implement the Display or Debug traits.
// - To print for debugging, derive Debug: #[derive(Debug)]
// - Use {:?} or {:#?} in println! for debug formatting.

#[derive(Debug)]
struct RectangleDebug {
    width: u32,
    height: u32,
}

fn ch5_2_debug_print_demo() {
    let rect1 = RectangleDebug {
        width: 30,
        height: 50,
    };

    // Debug print (single line)
    println!("rect1 is {:?}", rect1);

    // Pretty debug print (multi-line)
    println!("rect1 is {:#?}", rect1);

    // Using dbg! macro: prints to stderr with file/line info
    let scale = 2;
    let rect2 = RectangleDebug {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect2);
}

// -- Summary --
// - Structs group related data with names for each field, improving code clarity and safety.
// - Deriving Debug and using {:?} or dbg! helps with debugging.
// - Borrowing structs as function parameters avoids moving ownership.

// Next: Move the area function into a method on Rectangle for better encapsulation.

/// Run all demos from this section
pub fn ch5_2() {
    println!("--- Rectangle struct area calculation ---");
    ch5_2_basic_struct_demo();

    println!("\n--- Debug printing Rectangle ---");
    ch5_2_debug_print_demo();
}

