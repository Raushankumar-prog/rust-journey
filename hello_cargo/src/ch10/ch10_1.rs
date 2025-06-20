// Chapter 10.1: Generic Data Types - Notes & Examples

pub fn ch10_1() {
    // 1. What are Generics?
    // ---------------------
    // - Generics allow definition of functions, structs, enums, and methods that work with many concrete types.
    // - Avoid code duplication, increase flexibility.

    // 2. Generics in Functions
    // ------------------------
    // Example: Largest element in a slice (with trait bound)
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    // 3. Generics in Structs
    // ----------------------
    struct Point<T> {
        x: T,
        y: T,
    }
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // Using different types for x and y:
    struct Point2<T, U> {
        x: T,
        y: U,
    }
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 1.0, y: 4.0 };
    let integer_and_float = Point2 { x: 5, y: 4.0 };

    // 4. Generics in Enums
    // --------------------
    enum Option<T> {
        Some(T),
        None,
    }
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    // 5. Generics in Methods
    // ----------------------
    struct Point3<T> {
        x: T,
        y: T,
    }
    impl<T> Point3<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    impl Point3<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    // Mixup method with different generic parameter names
    struct Point4<X1, Y1> {
        x: X1,
        y: Y1,
    }
    impl<X1, Y1> Point4<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point4<X2, Y2>) -> Point4<X1, Y2> {
            Point4 {
                x: self.x,
                y: other.y,
            }
        }
    }
    let p1 = Point4 { x: 5, y: 10.4 };
    let p2 = Point4 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2); // p3.x = 5, p3.y = 'c'

    // 6. Generics & Performance (Monomorphization)
    // --------------------------------------------
    // - At compile time, Rust generates specialized code for each concrete type used with generics.
    // - No runtime cost for using generics.
    // Example:
    let integer = Some(5);   // Option<i32>
    let float = Some(5.0);   // Option<f64>
    // Rust generates Option_i32 and Option_f64 behind the scenes.

    println!("See source for notes and examples on generic data types in Rust.");
}