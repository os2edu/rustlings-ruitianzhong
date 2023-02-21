// macros2.rs
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
macro_rules! hello_macro{
    ()=>{
     println!("hello world");
    };
}
fn main() {
    my_macro!();
    hello_macro!();
}


