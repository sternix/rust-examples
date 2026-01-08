/*

in the method call k.print(), k might be a Kisi, a reference of type &Kisi,
or a smart pointer of type Box<Kisi> or Rc<Kisi>,
The print method might take the k either by value or by reference.
The same .print() sytax works in all cases, because Rust's . operator automatically
dereferences k or borrows a reference to it as needed.

*/

struct Kisi {}

impl Kisi {
    fn print(&self) {
        println!("Kisi.print");
    }
}

fn main() {
    let k = Kisi {};
    k.print();

    let rk = &k;
    rk.print();

    let bk = Box::new(&k);
    bk.print();

    let rck = std::rc::Rc::new(k);
    rck.print();
}
