// Chapter 7: Modules, Crates, Packages - Detailed Guide

pub fn ch7() {
    // 1. Crates & Packages
    // ---------------------
    // - Crate: Smallest compilation unit in Rust.
    //   * Library crate: No main(), reusable logic.
    //   * Binary crate: Has main(), builds to executable.
    // - Package: Bundle with Cargo.toml, can contain:
    //   * 1 library crate (src/lib.rs)
    //   * 0 or more binary crates (src/main.rs, src/bin/*.rs)
    //   * At least one crate required.

    // Example: cargo new my-project
    // my-project/
    //   Cargo.toml
    //   src/
    //     main.rs   // (binary crate root)

    // 2. Module System
    // ------------------
    // - Modules organize code, control scope and privacy.
    // - Defined with `mod` keyword, can be inline or in files.
    // - Crate root: src/lib.rs or src/main.rs

    // Example module tree
    // crate
    // └── front_of_house
    //     ├── hosting
    //     └── serving

    // 3. Declaring Modules
    // --------------------
    // - In crate root: `mod garden;` // looks for src/garden.rs or src/garden/mod.rs
    // - In other files: `mod veg;` in src/garden.rs looks for src/garden/veg.rs

    // Example:
    // src/lib.rs
    // mod garden; // garden.rs or /garden/mod.rs
    // pub fn eat_at_restaurant() {
    //     crate::garden::veg::plant();
    // }

    // src/garden.rs
    // pub mod veg; // veg.rs or /veg/mod.rs

    // src/garden/veg.rs
    // pub fn plant() {}

    // 4. Paths to Items
    // -----------------
    // - Absolute path: starts from crate root (crate::...)
    // - Relative path: starts from current module (mod_name::...)
    // - Use `super::` to reference parent module

    // Example:
    // Absolute: crate::front_of_house::hosting::add_to_waitlist();
    // Relative: front_of_house::hosting::add_to_waitlist();
    // Parent:   super::deliver_order();

    // 5. Privacy Rules
    // ----------------
    // - Everything is private by default.
    // - Use `pub` to make items public.
    // - Making a module pub doesn't make its contents public.
    // - Parent modules can't access private items of children, but children can access parents.

    // Example:
    // mod front_of_house {
    //     pub mod hosting {
    //         pub fn add_to_waitlist() {}
    //     }
    // }

    // 6. pub for Structs & Enums
    // --------------------------
    // - pub struct: struct is public, but fields are private unless also pub.
    // - pub enum: all variants are public.

    // Example:
    // pub struct Breakfast {
    //     pub toast: String,
    //     seasonal_fruit: String,
    // }
    // pub enum Appetizer { Soup, Salad }

    // 7. The use Keyword
    // ------------------
    // - Shortens paths in scope.
    // - Idiomatic: bring modules in scope, not functions (except with aliasing or to avoid conflicts).

    // use crate::front_of_house::hosting;
    // hosting::add_to_waitlist();

    // - `use` applies only to current scope. For submodules, repeat or use super::.

    // 8. The as Keyword
    // -----------------
    // - Rename imports locally to avoid conflicts.

    // Example:
    // use std::io::Result as IoResult;

    // 9. pub use (Re-Exporting)
    // -------------------------
    // - Combines pub and use for API design; exposes internal items with a cleaner path.

    // Example:
    // pub use crate::front_of_house::hosting;
    // Now, external code can use: my_crate::hosting::add_to_waitlist();

    // 10. Nested use, Glob operator
    // -----------------------------
    // - Nested: use std::{cmp::Ordering, io};
    // - Self in nested: use std::io::{self, Write};
    // - Glob: use std::collections::*;

    // 11. File Structure for Modules
    // ------------------------------
    // - mod foo; in crate root: looks for src/foo.rs or src/foo/mod.rs
    // - mod bar; in foo.rs: looks for src/foo/bar.rs
    // - Only declare mod once per module tree.

    // 12. Best Practices
    // ------------------
    // - Use modules for logical grouping and privacy.
    // - Use `pub use` for API re-export.
    // - Use absolute paths for flexibility.
    // - Struct fields private by default, use constructors for encapsulation.

    // 13. External Crates
    // -------------------
    // - Add to Cargo.toml (e.g., rand = "0.8.5")
    // - use rand::Rng;

    // 14. Example: Idiomatic use for std::collections::HashMap
    // --------------------------------------------------------
    // use std::collections::HashMap;
    // let mut map = HashMap::new();

    println!("Detailed module system notes and idiomatic patterns available in source.");
}