enum E {
    X,
    Y(i32),                  // tuple variant
    Z { f1: i32, f2: char }, // struct variant
}

impl E {
    fn print(&self) {
        match self {
            E::X => println!("X varyasyonu"),
            E::Y(n) => println!("Y varyasyonu değeri {}", n),
            E::Z { f1, f2 } => println!("Z varyasyonu f1: {}, f2: {}", f1, f2),
        }
    }
}

fn main() {
    let e = E::X;
    e.print();

    let e = E::Y(5);
    e.print();
    let e = E::Z { f1: 6, f2: 'x' };
    e.print();

    println!("hafızada {} byte yer kaplıyor", std::mem::size_of::<E>()); // ilginç olarak 8 byte yer kaplıyor
}
