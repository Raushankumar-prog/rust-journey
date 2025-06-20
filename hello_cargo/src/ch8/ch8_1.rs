// Chapter 8 (Section: Vectors) - Notes & Examples

pub fn ch8_1() {
    // 1. What is a Vector?
    // --------------------
    // - Vec<T> is a growable list type storing values of type T, contiguously in memory.
    // - All elements must be of the same type.

    // 2. Creating Vectors
    // -------------------
    // Empty with explicit type:
    let v1: Vec<i32> = Vec::new();
    // With initial values (type inferred):
    let v2 = vec![1, 2, 3];

    // 3. Updating a Vector
    // --------------------
    // Use `push` to append elements.
    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);

    // 4. Reading Elements
    // -------------------
    let v = vec![1, 2, 3, 4, 5];
    // (a) Indexing (panics if out of bounds)
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // (b) get() method (safe, returns Option)
    let third = v.get(2);
    match third {
        Some(val) => println!("The third element is {val}"),
        None => println!("There is no third element."),
    }

    // (c) Out of bounds
    // let x = &v[100]; // panics!
    // let y = v.get(100); // returns None

    // 5. Borrowing Rules with Vectors
    // -------------------------------
    // Can't have immutable and mutable borrows at the same time.
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6); // error: cannot borrow as mutable because it is also borrowed as immutable

    // 6. Iterating Over Vectors
    // -------------------------
    let v = vec![100, 32, 57];
    for i in &v {
        println!("Immutable ref: {i}");
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("After mutation: {:?}", v);

    // 7. Vectors of Multiple Types with Enums
    // ---------------------------------------
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // 8. Dropping Vectors
    // -------------------
    {
        let v = vec![1, 2, 3, 4];
        // v and its contents are dropped at end of scope
    }

    // 9. Miscellaneous
    // ----------------
    // - Use Vec<T> when you need a growable, ordered list.
    // - Many methods: pop(), len(), is_empty(), sort(), etc. (see Vec<T> docs)

    println!("See source for notes and examples on vectors (Vec<T>) in Rust.");
}