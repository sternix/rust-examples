trait Bir {
    fn method(&self);
}

trait Iki {
    fn method(&self);
}

struct A;

impl Bir for A {
    fn method(&self) {
        println!("A->Bir");
    }
}

impl Iki for A {
    fn method(&self) {
        println!("A->Iki");
    }
}

fn main() {
    let a = A;

    // HATA
    //multiple applicable items in scope
    //multiple `method` found

    // a.method();

    Bir::method(&a);
    Iki::method(&a);
}
