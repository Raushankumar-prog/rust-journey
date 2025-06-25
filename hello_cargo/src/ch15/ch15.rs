// Chapter 15.1: Box, Rc, RefCell, and Smart Pointers - Notes & Examples

// --- Using Box<T> to Point to Data on the Heap ---
// - Box<T> allocates data on the heap, leaving a pointer on the stack.
// - Used when you:
//   * Need a type whose size can't be known at compile time (e.g. recursive types)
//   * Want to transfer ownership of large data without copying it
//   * Want to own a value of any type implementing a trait (trait objects, see Chapter 18)

fn box_example() {
    let b = Box::new(5);
    println!("b = {b}");
}

// --- Enabling Recursive Types with Boxes ---
// - Rust needs to know the size of types at compile time.
// - Recursive types (like a linked list) have unknown size unless you use indirection (Box, Rc, etc).
// - Example: Cons list (linked list)

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn recursive_type_example() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // This works because Box<List> has known size (pointer), breaking the infinite chain.
}

// --- The Deref Trait: Treating Smart Pointers like References ---
// - Box<T> implements Deref, so *box dereferences to the value.
// - You can implement Deref for your own types (like a custom smart pointer).

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> { MyBox(x) }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.0 }
}

fn deref_example() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, *y);
}

// --- Drop Trait: Running Code on Cleanup ---
// - Drop lets you specify what happens when a value goes out of scope (destructor).
// - Rust automatically calls drop; you can't call it manually, but you can use std::mem::drop to drop early.

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn drop_example() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
    // d and then c are dropped at end of scope (in reverse order).
    // To drop early: std::mem::drop(c);
}

// --- Rc<T>: Reference Counting for Shared Ownership ---
// - Rc<T> enables multiple ownership of heap data (single-threaded only).
// - Rc::clone(&rc) increments reference count (does not deep copy).
// - Data is cleaned up when reference count is zero.

use std::rc::Rc;

enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}

use RcList::{RcCons, RcNil};

fn rc_example() {
    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = RcCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = RcCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

// --- RefCell<T> and the Interior Mutability Pattern ---
// - RefCell<T> allows "interior mutability": mutate data through an immutable reference (checked at runtime).
// - borrow()/borrow_mut() create tracked references (panics at runtime if violated).
// - Example use case: Writing tests with mock objects.

use std::cell::RefCell;

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker { messenger, value: 0, max }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]) }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

// --- Summary of Smart Pointers ---
//
// * Box<T>: Single ownership, heap allocation, compile-time borrowing.
// * Rc<T>: Multiple ownership, heap allocation, compile-time immutable borrowing, single-threaded.
// * RefCell<T>: Single ownership, interior mutability, borrowing checked at runtime, single-threaded.
// * Use Box<T> for recursive types and heap allocation; Rc<T> for shared ownership; RefCell<T> for interior mutability.
//
