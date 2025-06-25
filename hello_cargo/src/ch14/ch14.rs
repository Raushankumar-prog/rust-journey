// Chapter 14: Publishing and Working with Cargo - Notes & Examples

// =================================================
// Customizing Builds with Release Profiles
// =================================================
//
// - Cargo has two main profiles: dev (cargo build) and release (cargo build --release).
// - Each profile has a set of default settings, which you can override in Cargo.toml.
// - Example: Controlling optimization level (opt-level: 0 = none, 3 = max):
//
// [profile.dev]
// opt-level = 1
//
// [profile.release]
// opt-level = 3
//
// - Use dev profile for fast, unoptimized builds (development).
// - Use release profile for optimized, slower-to-compile builds (production).

// =================================================
// Publishing a Crate to Crates.io
// =================================================
//
// 1. Add documentation comments (///) to your public API.
//    - Use Markdown in comments.
//    - Example:
//
//    /// Adds one to the number given.
//    ///
//    /// # Examples
//    /// ```
//    /// let answer = my_crate::add_one(5);
//    /// assert_eq!(6, answer);
//    /// ```
//    pub fn add_one(x: i32) -> i32 { x + 1 }
//
// 2. Generate docs with cargo doc or cargo doc --open.
//    - Documentation comments (///) for items.
//    - Inner doc comments (//! at top of file/module) document the crate/module as a whole.
//
// 3. Use pub use to re-export items and customize your public API structure.
//
//    // Example: Top-level re-exports
//    pub use self::kinds::PrimaryColor;
//    pub use self::kinds::SecondaryColor;
//    pub use self::utils::mix;
//
// 4. Set up a crates.io account, get your API key, and run cargo login.
//
// 5. Add required metadata to your Cargo.toml:
//
//    [package]
//    name = "guessing_game"
//    version = "0.1.0"
//    edition = "2024"
//    description = "A fun game where you guess what number the computer has chosen."
//    license = "MIT OR Apache-2.0"
//
// 6. Publish with cargo publish.
//    - Once a version is published, it cannot be deleted, only yanked (preventing future use).
//    - Yank a version: cargo yank --vers 1.0.1
//    - Undo yank: cargo yank --vers 1.0.1 --undo

// =================================================
// Cargo Workspaces
// =================================================
//
// - A workspace is a set of crates with a shared Cargo.lock and target directory.
// - Add a [workspace] section to top-level Cargo.toml. List members.
//
// [workspace]
// resolver = "3"
// members = ["adder", "add_one"]
//
// - Each crate/package in workspace has its own Cargo.toml, but dependencies are resolved together.
// - Dependencies between crates in the workspace must be listed in the dependent crate's Cargo.toml as path dependencies.
//
// - Example structure:
//   add/
//     Cargo.toml   (workspace root)
//     adder/       (binary crate)
//     add_one/     (library crate)
//
// - Build all with `cargo build` at workspace root.
// - Run a binary: `cargo run -p adder`.
// - Add external dependencies to individual crates as needed.
//
// - Tests and publishing:
//   - `cargo test` runs tests for all crates in the workspace.
//   - `cargo test -p add_one` runs tests only for add_one crate.
//   - Each crate must be published separately with `cargo publish -p crate_name`.

// =================================================
// Installing Binaries with cargo install
// =================================================
//
// - Install binary crates (with main.rs or other binary target) locally:
//   cargo install ripgrep
// - Binaries are placed in $HOME/.cargo/bin (should be in your PATH).
// - Only binary crates can be installed this way (not library-only crates).

// =================================================
// Extending Cargo with Custom Commands
// =================================================
//
// - Add a binary named `cargo-something` to your PATH, and you can run it as `cargo something`.
// - Custom subcommands show up in `cargo --list`.
//
// Example: `cargo install cargo-edit` makes `cargo add` available.

// =================================================
// Summary
// =================================================
//
// - Cargo makes it easy to share, document, and publish Rust code.
// - Use workspaces for multi-crate projects.
// - Use release profiles for fine-tuning builds.
// - Document your code using Markdown comments (///, //!).
// - Publish to crates.io with proper metadata and licensing.
// - Use pub use to shape your public API.
// - Install binaries with cargo install.
// - Extend Cargo with custom subcommands.

fn ch14() {
    println!("See source for full notes and examples on publishing, workspaces, and customizing Rust projects with Cargo.");
}