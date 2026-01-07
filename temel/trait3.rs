trait Hash {
    fn hash(&self) -> u64;
}

impl Hash for bool {
    fn hash(&self) -> u64 {
        if *self { 0 } else { 1 }
    }
}

impl Hash for i64 {
    fn hash(&self) -> u64 {
        *self as u64
    }
}

fn main() {
    let b = true;
    println!("{}", b.hash());

    let i = 1234_i64;
    println!("{}", i.hash());
}
