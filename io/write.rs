/*
Console output without macro
println!("Hello, world!"); yerine
*/

// use Write trait that contains write() function
use std::io::Write;

fn main() {
    std::io::stdout().write(b"Hello, world!\n").unwrap();
}
