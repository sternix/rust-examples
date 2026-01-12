use std::collections::HashSet;
use std::iter::FromIterator;

fn vec_to_set(vec: Vec<u8>) -> HashSet<u8> {
    HashSet::from_iter(vec)
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let set = vec_to_set(v);
    for i in set {
        println!("{i}");
    }
}
