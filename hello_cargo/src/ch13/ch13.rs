// Chapter 13: Closures and Iterators - Notes & Examples

// =================================================
// Closures: Anonymous Functions That Capture Their Environment
// =================================================
//
// Closures are anonymous functions you can save in a variable or pass as arguments to other functions.
// They can capture values from their environment.
//
// Key Features:
// - Can capture variables from their defining scope (environment).
// - Useful for customizing behavior, especially with iterator methods.
// - Syntax is concise and often omits type annotations (inferred).
// - Can take ownership, borrow mutably, or borrow immutably from the environment.

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    // giveaway uses a closure as argument to Option::unwrap_or_else
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // Closure captures &self and can call methods on it.
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue { ShirtColor::Red } else { ShirtColor::Blue }
    }
}

fn ch13() {
    println!("=== Closure Examples ===");

    // Example: Closure capturing self
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);

    // Closure type syntax and inference:
    // - Closures can have explicit or inferred parameter/return types
    let add_one_v1 = |x: u32| -> u32 { x + 1 };
    let add_one_v2 = |x| x + 1;
    println!("add_one_v1(5) = {}, add_one_v2(10) = {}", add_one_v1(5), add_one_v2(10));

    // Closure captures immutable reference
    let list = vec![1, 2, 3];
    let only_borrows = || println!("From closure: {:?}", list);
    only_borrows();

    // Closure captures mutable reference
    let mut list2 = vec![1, 2, 3];
    let mut borrows_mutably = || list2.push(7);
    borrows_mutably();
    println!("After mut closure: {:?}", list2);

    // Move closure for threads
    use std::thread;
    let list3 = vec![1, 2, 3];
    thread::spawn(move || println!("From thread: {:?}", list3)).join().unwrap();

    // Closure type is inferred from first call:
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // let n = example_closure(5); // Would not compile, type is String

    // Theory:
    // - Closures can capture environment in three ways:
    //   1. By immutable reference (&T) - default if only read.
    //   2. By mutable reference (&mut T) - if closure mutates environment.
    //   3. By value (T, "move") - if closure takes ownership.
    // - The way a closure captures environment determines which Fn traits it implements:
    //   - FnOnce: can be called once, may move out of environment.
    //   - FnMut: can be called multiple times, may mutate environment.
    //   - Fn: can be called multiple times, does not mutate environment.
    // - The compiler infers how closures capture environment and which trait is needed depending on usage (e.g. in threads, iterator methods).
}

// =================================================
// Iterators: Processing a Series of Items
// =================================================
//
// Iterators allow you to process a sequence of items in turn, supporting both functional and imperative styles.
//
// Key Features:
// - Created by calling .iter(), .iter_mut(), or .into_iter() on collections.
// - The Iterator trait requires implementing the next() method.
// - Iterators are lazy: nothing happens until a consuming adapter (like sum, collect, for, etc.) is called.
// - Adapters (like map, filter) create new iterators, which can be chained.

fn iterator_examples() {
    println!("\n=== Iterator Examples ===");

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {val}");
    }

    // The Iterator trait and next:
    let v2 = vec![1, 2, 3];
    let mut v2_iter = v2.iter();
    assert_eq!(v2_iter.next(), Some(&1));
    assert_eq!(v2_iter.next(), Some(&2));
    assert_eq!(v2_iter.next(), Some(&3));
    assert_eq!(v2_iter.next(), None);

    // Consuming adapters: sum()
    let v3 = vec![1, 2, 3];
    let v3_iter = v3.iter();
    let total: i32 = v3_iter.sum();
    assert_eq!(total, 6);

    // Iterator adapters: map, filter (lazily builds up a new iterator)
    let v4: Vec<i32> = vec![1, 2, 3];
    let v5: Vec<_> = v4.iter().map(|x| x + 1).collect();
    assert_eq!(v5, vec![2, 3, 4]);
    let v6: Vec<_> = v5.into_iter().filter(|x| *x % 2 == 0).collect();
    assert_eq!(v6, vec![2, 4]);

    // Theory:
    // - Consuming adapters: sum(), collect(), for loops, etc. (consume iterator)
    // - Iterator adapters: map, filter, take, skip, etc. (create new iterators)
    // - Chaining: Adapters can be chained for complex logic.
    // - Zero-cost abstraction: Compiler optimizes away the abstraction.
    // - Iterator methods often take closures for customization.
}

// =================================================
// Closures with Iterators: Filtering Example
// =================================================
//
// Closures are commonly used with iterators to customize how items are processed.

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // `filter` takes a closure that captures shoe_size from its environment.
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn filter_shoes_example() {
    println!("\n=== Filtering Shoes Example ===");
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];
    let in_my_size = shoes_in_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
    println!("Shoes in my size: {:?}", in_my_size);
}

// =================================================
// Improving Code with Iterators (Theory)
// =================================================
//
// - Iterators are lazy: they do nothing until consumed (by sum, collect, for, etc).
// - Iterator adapters (map, filter, etc) create new iterators, can be chained.
// - Consuming adapters (sum, collect) run the chain.
// - Closures can capture environment and are common in iterator chains.
// - Using iterators often makes code more concise and clear than explicit for loops.
// - Performance: iterator chains are zero-cost abstractions; compiler optimizes to same or better code than for-loops.
// - Real world: Iterators are heavily used for complex data processing, e.g. filter/map/collect pipelines, parsing, etc.
//
// Example: (Functional style code)
// let results: Vec<&str> = contents
//     .lines()
//     .filter(|line| line.contains(query))
//     .collect();
//
// This is clearer and less error-prone than writing a for loop and pushing into a Vec manually.
