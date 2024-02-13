// Crates are the smallest amount of code the Rust compiler considers at a time.
// Binary Crates: programs that can compile into an executable.
// Library Crates: don't compile into an executable, but contains functionality to be shared with multiple projects.

// Packages are bundles of one or more crates.
// Packages can contain many crates but only one library.

// How module compiler works: https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html

// To write "Asparagus" instead of "garden::vegetables::Asparagus", you can use "use" in that scope.
// use crate::garden::vegetables::Asparagus;

// Create new libraries with "cargo new restaurant --lib"

use crate::enums;
use crate::guess_the_number::start_guess_the_number;

// Grouping code into modules.
// This is like pragma region in terms of the readability aspect.
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// Paths
// Absolute paths start from the crate root.
// Relative paths start from the current module and uses self, super, or an identifier in the current module.
// // super is like using .. syntax.

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // Here we use super to access a function from the parent module.
    }

    fn cook_order() {}
}

fn path() {
    crate::guess_the_number::start_guess_the_number();
}

// Best practice for packages.
// Put public things in the binary crate and keep the library crate private.
// The binary crate uses the library just how other external crates would.

// We can rename to avoid confusion.
use std::fmt::Result;
use std::io::Result as IoResult;

// Re-exporting: https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html
pub use crate::front_of_house::hosting;

// We can combine paths for readability.
use std::{cmp::Ordering, io};
use std::io::{self, Write}; // (Error here because we are importing io twice).

// Glob operator.
use std::collections::*; // This brings all public items in std::collections into scope.

// Separating modules into different files: https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
