trait X {
    fn x(&self);
}

struct S;

impl X for S {
    fn x(&self) {
        println!("X");
    }
}

fn d(x: &dyn X) {
    x.x();
}

fn main() {
    let s = S;
    d(&s);

    // unit tipi olduğundan
    d(&S);

    // Object cast
    <S as X>::x(&s);

    // Trait'in metodu
    X::x(&s);

    // Struct'ın impl metodu
    S::x(&s);
}
