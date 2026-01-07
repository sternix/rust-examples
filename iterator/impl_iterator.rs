struct TI(i32);

impl Iterator for TI {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 += 1;
        Some(self.0)
    }
}

fn main() {
    for i in TI(0).take(10) {
        println!("{}", i);
    }
}
