// ===================================================
// Topic: Enums, Pattern Matching, and Option in Rust
// ===================================================
//
// Purpose:
// --------
// - To understand how to use enums to represent a value that can be one of several possible types (variants).
// - To see how pattern matching (with `match`, `if let`, and `let ... else`) lets you write expressive, type-safe code that handles each case.
// - To learn about Option<T> and how Rust avoids null values, enforcing handling of the "no value" case.
//
// Concept Outline:
// ----------------
// 1. Why Enums? How are they different from structs?
// 2. Defining and using enums.
// 3. Enums with data (variants can hold data of different types).
// 4. The Option<T> enum for "an optional value" (replaces null).
// 5. Pattern matching with match (exhaustive, safe).
// 6. Extracting data from enum variants with match arms.
// 7. Catch-all patterns: variable and _ (wildcard).
// 8. Concise matching: if let and let ... else.
// 9. Using enums with methods, and Option<T> in real code.
//
// ===========================================================================
// 1. Why Enums? -- "A value of one of several possible kinds"
// ===========================================================================

// Example: IP addresses are either V4 or V6 (never both at once)
enum IpAddrKind {
    V4,
    V6,
}

// You can use enums in function parameters:
fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("Routing IPv4 address..."),
        IpAddrKind::V6 => println!("Routing IPv6 address..."),
    }
}

// ===========================================================================
// 2. Enums with Data: Variants Holding Data (more concise than struct+enum)
// ===========================================================================

// Approach 1: struct + enum
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// Approach 2: Data inside enum variant
enum IpAddr2 {
    V4(String),
    V6(String),
}

// Variants can have different data types
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// ===========================================================================
// 3. Enum Variants Can Hold Any Type, Even Structs or Other Enums
// ===========================================================================
struct Ipv4Addr {/* fields omitted */}
struct Ipv6Addr {/* fields omitted */}
enum StdIpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// ===========================================================================
// 4. Example: Enum with Multiple Variant Types (like sum types)
// ===========================================================================
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Methods can be implemented for enums too!
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message"),
            Message::Move { x, y } => println!("Move to ({x}, {y})"),
            Message::Write(text) => println!("Write message: {text}"),
            Message::ChangeColor(r, g, b) => println!("Change color to ({r}, {g}, {b})"),
        }
    }
}

// ===========================================================================
// 5. The Option<T> Enum: Rust's Safe Alternative to Null
// ===========================================================================
enum Option<T> {
    None,
    Some(T),
}

// Rust's std library version of Option is used like this:
fn option_examples() {
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    // You can't use Option<T> as if it were T directly
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y; // error
}

// ===========================================================================
// 6. Pattern Matching with match: Exhaustive, Safe, and Powerful
// ===========================================================================
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Arms can run blocks and extract data:
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... more states can be added
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents2(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

// Matching Option<T> to extract and transform values
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// match must be exhaustive
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//     } // error: non-exhaustive patterns: `None` not covered
// }

// ===========================================================================
// 7. Catch-All Patterns: variable and _ (wildcard) for default actions
// ===========================================================================
fn match_catchall_examples() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // Use _ if you don't care about the value
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    // Do nothing for all others
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // do nothing for all others
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(_spaces: u8) {}
fn reroll() {}

// ===========================================================================
// 8. Concise Control Flow: if let and let ... else
// ===========================================================================

fn concise_match_examples() {
    // if let: concise single-pattern match for Option and enums
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // Equivalent to:
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // With else:
    let mut count = 0;
    let coin = Coin2::Quarter(UsState::Alaska);
    if let Coin2::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    // let ... else: for "happy path" and early return
    let coin2 = Coin2::Quarter(UsState::Alabama);
    let description = describe_state_quarter(coin2);
    println!("Description: {:?}", description);
}

fn describe_state_quarter(coin: Coin2) -> Option<String> {
    let Coin2::Quarter(state) = coin else {
        return None;
    };
    if matches!(state, UsState::Alabama | UsState::Alaska) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

// ===========================================================================
// Code Demo: Enums, Pattern Matching, Option
// ===========================================================================

pub fn ch6() {
    // Basic enum usage
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    // Enum with data
    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));
    match home {
        IpAddr2::V4(addr) => println!("Home IPv4: {addr}"),
        IpAddr2::V6(addr) => println!("Home IPv6: {addr}"),
    }
    match loopback {
        IpAddr2::V4(addr) => println!("Loopback IPv4: {addr}"),
        IpAddr2::V6(addr) => println!("Loopback IPv6: {addr}"),
    }

    // Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six: {:?}, none: {:?}", six, none);

    // match with Coin
    let c = Coin::Penny;
    println!("Penny value: {}", value_in_cents(c));

    // match with Coin2 and UsState
    let quarter = Coin2::Quarter(UsState::Alaska);
    println!("Quarter value: {}", value_in_cents2(quarter));

    // Message enum method
    let msg = Message::Move { x: 10, y: 20 };
    msg.call();

    // Demo concise matching
    concise_match_examples();

    // Demo match catchall
    match_catchall_examples();
}

