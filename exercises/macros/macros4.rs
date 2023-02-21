// macros4.rs
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a hint.


macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
    ($a:expr,$b:expr) => {
        println!("{} {}", $a, $b);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
    my_macro!("hello", "world");
}
