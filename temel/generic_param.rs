use std::fmt::Debug;

fn f(a: &dyn Debug, b: &dyn Debug) {
    todo!()
}

fn g<T: Debug>(a: &T, b: &T) {
    todo!()
}

#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B;

fn main() {
    let a = A;
    let b = B;
    // bunda sorun yok
    f(&a, &b);

    // bu hata veriyor
    // iki paramtetre'de aynı tip olmalı,
    // Debug olması yetmiyor
    g(&a, &b);
    /*
    26 |     g(&a, &b);
       |     -     ^^ expected `&A`, found `&B`
       |     |
       |     arguments to this function are incorrect
       */
}
