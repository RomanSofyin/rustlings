// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.


// use self::macros::{my_macro};
use self::macros::my_macro;
// use macros::my_macro as my_macro;

// #[macro_export]
mod macros {
    // extern mod macros;

    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    pub(crate) use my_macro;
}

fn main() {
    my_macro!();
}
