use std::ops::Index;

fn main() {
    let a = [1, 2, 3, 4];

    println!("{}", *a.index(1));
}