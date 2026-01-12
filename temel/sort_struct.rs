// struct'ları sıralamak için

use std::cmp::Ordering;

#[derive(Debug, Eq)]
struct X {
    f: i32,
}

impl std::cmp::Ord for X {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f.cmp(&other.f)
    }
}

impl std::cmp::PartialOrd for X {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for X {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

fn main() {
    let mut a = [X { f: 5 }, X { f: 4 }, X { f: 2 }, X { f: 3 }, X { f: 1 }];
    a.sort();
    println!("{:?}", a);
}
