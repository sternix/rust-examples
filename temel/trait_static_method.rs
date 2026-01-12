/*
In most object-oriented languages, interfaces can't include static methods or constructors.
However, Rust traits can include static methods and constructors
*/

trait X {
    fn new() -> Self;
    fn static_method();
    fn instance_method(&self);
}

struct A;

impl X for A {
    // constructor
    fn new() -> Self {
        A {}
    }

    fn static_method() {
        println!("Static method");
    }

    fn instance_method(&self) {
        println!("Instance method");
    }
}

fn main() {
    A::static_method();

    let a = A::new();
    a.instance_method();
}
