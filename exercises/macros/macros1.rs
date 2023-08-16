// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.



macro_rules! my_macro {
    (1) => {
        println!("Matched 1");
    };
    (2) => {
        println!("Matched 2");
    };
    ($expr:expr) => {
        println!("Matched expression: {}", $expr);
    };

    // () => {
    //     println!("Check out my macro!");
    // };
}

fn main() {
    my_macro!(2);
}
