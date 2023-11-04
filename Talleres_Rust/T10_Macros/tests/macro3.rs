// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

#[macro_use]//add this to local macro
mod macros {
    //#[macro_export] add this to public macro
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
