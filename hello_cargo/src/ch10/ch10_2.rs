// Chapter 10.2: Traits - Defining Shared Behavior - Notes & Examples

pub fn ch10_2() {
    // 1. What is a Trait?
    // -------------------
    // - Traits define shared behavior: a set of method signatures that types can implement.
    // - Similar to "interfaces" in other languages.

    // 2. Defining a Trait
    // -------------------
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    // 3. Implementing a Trait for Types
    // ---------------------------------
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct SocialPost {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub repost: bool,
    }
    impl Summary for SocialPost {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    // 4. Default Implementations in Traits
    // ------------------------------------
    pub trait SummaryDefault {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }
    // Can override or keep default. Default implementations can call other required trait methods.

    // 5. Traits as Parameters & Trait Bounds
    // --------------------------------------
    // (a) Using impl Trait syntax:
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    // (b) Using explicit trait bounds (generic):
    pub fn notify_bound<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }
    // (c) Multiple trait bounds:
    // pub fn notify<T: Summary + Display>(item: &T) { ... }

    // 6. Where Clauses for Clearer Trait Bounds
    // -----------------------------------------
    // fn some_function<T, U>(t: &T, u: &U) -> i32
    // where
    //     T: Display + Clone,
    //     U: Clone + Debug,
    // { ... }

    // 7. Returning Types that Implement Traits
    // ---------------------------------------
    fn returns_summarizable() -> impl Summary {
        SocialPost {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            repost: false,
        }
    }
    // Note: Only one concrete return type allowed with impl Trait.

    // 8. Conditionally Implementing Methods with Trait Bounds
    // -------------------------------------------------------
    use std::fmt::Display;
    struct Pair<T> {
        x: T,
        y: T,
    }
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    // 9. Blanket Implementations
    // -------------------------
    // Standard library: impl<T: Display> ToString for T
    let s = 3.to_string(); // works because i32 implements Display

    // 10. Orphan Rule and Coherence
    // -----------------------------
    // - You can implement a trait for a type only if either the trait or the type is local to your crate.

    println!("See source for notes and examples on traits and shared behavior in Rust.");
}