/*
burada dikkatimi çeken incr fonksiyonunda &mut olması
ve main'de mut olarak tanımlanmış bir değişkeni incr fonksiyonuna &mut olarak
yollamak
*/

use std::fmt;

struct Foo {
    id: u32,
}

// yada
// #[derive(Debug)]
struct A(u32);

impl fmt::Display for A {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn incr_foo(x: &mut Foo) {
    x.id += 1
}

fn incr_a(x: &mut A) {
    x.0 += 1
}

fn incr(x: &mut f64) {
    *x += 1.0
}

fn main() {
    let mut f = 1.0;
    incr(&mut f);
    println!("result: {}", f);

    let mut foo = Foo { id: 1 };
    incr_foo(&mut foo);
    println!("foo.id = {}", foo.id);

    let mut a = A(1);
    incr_a(&mut a);
    println!("A: {}", a);
}
