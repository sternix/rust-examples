#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
    Purple,
}

use Color::*;

fn main() {
    let a: [Color; 4] = [Red, Blue, Green, Purple];
    dbg!(a);
}
