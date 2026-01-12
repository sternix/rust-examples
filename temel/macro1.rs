// https://twitter.com/DaveWellsted

macro_rules! mymacro {
    ($x:expr) => {
        println!("Hello {}", $x)
    };
}

fn main() {
    mymacro!("Rustaceans");
}
