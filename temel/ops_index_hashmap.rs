use std::collections::HashMap;
use std::ops::Index;

fn main() {
    let mut m = HashMap::new();
    m.insert("X", 10);
    m.insert("Y", 100);
    m.insert("Z", 1000);

    println!("{}", *m.index("X"));
    println!("{}", *m.index("Y"));
    println!("{}", *m.index("Z"));
}
