// ch5_3.rs

// Struct: Custom data type to group related fields
#[derive(Debug)] // Allows printing the struct with {:?}
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation block for methods and associated functions
impl Rectangle {
    // Method: calculates the area of the rectangle
    // Uses &self → immutable reference to self (the instance calling this method)
    // self: &Self is equivalent to &Rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method: checks if one rectangle can completely contain another
    // Takes &self and another &Rectangle as parameters (both immutable references)
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function: doesn't take &self
    // Works like a static method in other languages
    // Often used as constructor, returns a new Rectangle with equal width and height
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // Method with the same name as a field — demonstrates method vs field usage
    // Returns true if width is greater than 0
    fn width(&self) -> bool {
        self.width > 0
    }
}

pub fn ch5_3() {
    // Create instances of Rectangle
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // Method call syntax: rect1.area() is equivalent to Rectangle::area(&rect1)
    println!("Area of rect1: {} sq px", rect1.area());

    // Use can_hold method to compare rectangles
    // rect1 is the caller, rect2 and rect3 are arguments
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // true
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3)); // false

    // Associated function call using :: syntax
    // Creates a square where width = height = 20
    let sq = Rectangle::square(20);
    println!("Square: {:?}, Area: {}", sq, sq.area());

    // Demonstrating method vs field with same name
    // rect1.width() → calls the method
    // rect1.width → accesses the field
    if rect1.width() {
        println!("rect1 width field: {}", rect1.width);
    }

    // Note:
    // - Rust auto-dereferences when calling methods, so you can write rect.area() instead of (&rect).area()
    // - Methods are defined using `fn` inside `impl`, and the first parameter is always `self`, `&self`, or `&mut self`
    // - Associated functions don't take self and are called with `::` instead of `.`
}
