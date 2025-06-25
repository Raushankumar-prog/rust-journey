// Chapter 16: Using Threads, Message Passing, and Shared-State Concurrency - Notes & Examples

// =============================================================
// 1. Using Threads to Run Code Simultaneously
// =============================================================
//
// - Rust uses OS threads (1:1 mapping).
// - thread::spawn launches a new thread, takes a closure.
// - Threads may interleave unpredictably. Main thread ending will terminate all spawned threads.

use std::thread;
use std::time::Duration;

fn basic_thread_example() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Spawned thread: hi number {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("Main thread: hi number {i}");
        thread::sleep(Duration::from_millis(1));
    }
    // Spawned thread may not finish before main ends!
}

// =============================================================
// 2. Waiting for Threads: join Handles
// =============================================================
//
// - thread::spawn returns a JoinHandle<T>.
// - join() will block until the thread finishes.

fn join_handle_example() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Spawned thread: hi number {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("Main thread: hi number {i}");
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap(); // Ensures spawned thread completes
}

// =============================================================
// 3. Using move Closures with Threads
// =============================================================
//
// - move keyword moves ownership of captured variables into the closure/thread.

fn move_closure_example() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}

// =============================================================
// 4. Message Passing with Channels
// =============================================================
//
// - std::sync::mpsc::channel creates a sender/receiver pair (multiple producer, single consumer).
// - tx.send(val) moves val into the channel; rx.recv() waits for a value.

use std::sync::mpsc;

fn channel_example() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec!["hi", "from", "the", "thread"];
        for val in vals {
            tx.send(String::from(val)).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });
    for received in rx {
        println!("Got: {received}");
    }
}

// =============================================================
// 5. Multiple Producers
// =============================================================
//
// - Clone the sender to have multiple producers.

fn multi_producer_channel_example() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec!["one", "two"];
        for val in vals {
            tx1.send(String::from(val)).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });
    thread::spawn(move || {
        let vals = vec!["three", "four"];
        for val in vals {
            tx.send(String::from(val)).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });
    for received in rx {
        println!("Got: {received}");
    }
}

// =============================================================
// 6. Shared-State Concurrency with Mutex<T> and Arc<T>
// =============================================================
//
// - Mutex<T> provides mutual exclusion: only one thread accesses data at a time.
// - Arc<T> is an atomic reference-counted pointer for sharing ownership across threads.
// - Use Arc<Mutex<T>> for shared, mutable state.

use std::sync::{Arc, Mutex};

fn mutex_arc_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap()); // Should print 10
}

// =============================================================
// 7. Send and Sync Traits
// =============================================================
//
// - Send: Type's ownership can be safely transferred between threads.
// - Sync: Type can be referenced from multiple threads safely.
// - Most types are Send and Sync unless they contain non-thread-safe primitives (like Rc<T> or RefCell<T>).
// - Arc<T> and Mutex<T> are thread-safe; Rc<T> and RefCell<T> are NOT.

// =============================================================
// 8. Summary & Best Practices
// =============================================================
//
// - Use threads to run code simultaneously.
// - Use message passing (channels) to communicate safely between threads.
// - Use Arc<Mutex<T>> for shared, mutable state.
// - The type system and ownership rules help prevent data races and concurrency bugs.
// - Send and Sync traits are built into Rust for thread safety guarantees.

pub fn ch16() {
    println!("-- Basic thread example --");
    basic_thread_example();
    println!("-- Join handle example --");
    join_handle_example();
    println!("-- Move closure example --");
    move_closure_example();
    println!("-- Channel example --");
    channel_example();
    println!("-- Multi-producer channel example --");
    multi_producer_channel_example();
    println!("-- Mutex/Arc example --");
    mutex_arc_example();
    println!("See source for full notes and more concurrency examples in Rust.");
}