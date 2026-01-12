use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash<T: ?Sized + Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn main() {
    println!("{}", hash(&1));
    println!("{}", hash(&'A'));
    println!("{}", hash("hello"));
}
