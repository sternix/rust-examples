use std::cell::{Cell, RefCell};

/*
Cell sadece Copy tiplerde işe yarıyor,
String vs için RefCell kullanılıyor
*/

struct A {
    s: RefCell<String>,
    b: Cell<i32>,
}

impl A {
    fn new() -> Self {
        A {
            s: RefCell::new("".into()),
            b: Cell::new(0),
        }
    }

    fn print(&self) {
        println!("{}-{}", self.s.borrow(), self.b.get());
    }
}

fn main() {
    let a = A::new();
    a.print();

    a.b.set(28);
    a.print();

    a.s.replace("test".to_string());
    a.print();

    *a.s.borrow_mut() = "Hello".to_string();
    a.print();

    {
        // blok olmazsa panikliyor
        let mut s = a.s.borrow_mut();
        s.push_str("XYZ");
    }
    a.print();

    a.s.borrow_mut().push_str(" World");

    a.print();
}
