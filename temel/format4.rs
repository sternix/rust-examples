use std::fmt::{Display, Formatter, Result};

struct MyType(i32);

impl Display for MyType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    let x = MyType(19);
    println!("My Type: {}", x)
}
