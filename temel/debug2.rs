use std::fmt;

struct MyType(i32);

impl fmt::Display for MyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    let x = MyType(19);
    println!("My Type: {}", x)
}
