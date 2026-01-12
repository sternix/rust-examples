trait X {
    fn x(&self);
}

// burada X subtrait oluyor
trait Y: X {
    fn y(&self);
}

struct A;

// A hem X'i hem'de Y implement etmeli
impl X for A {
    fn x(&self) {
        println!("X.A");
    }
}

impl Y for A {
    fn y(&self) {
        println!("Y.A");
    }
}

fn main() {
    let a = A;
    a.x();
    a.y();
}
