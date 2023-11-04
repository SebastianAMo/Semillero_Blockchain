// macros2.rs
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}

/* macros must come before the function
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
 */