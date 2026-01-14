use std::ops::Index;
use std::ops::IndexMut;

fn main() {
    let mut v = vec!["A".to_string(), "B".to_string()];

    (*v.index_mut(0)).push_str("X");
    (*v.index_mut(1)).push_str("Y");

    println!("{}", *v.index(0));
    println!("{}", *v.index(1));
}
