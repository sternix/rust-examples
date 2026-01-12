// çoklu değer atama

#[derive(Debug)]
struct X {
    a: String,
    b: String,
}

fn main() {
    // atanan tip Copy ise işe yarıyor
    let x @ y @ z = 123;

    println!("{x},{y},{z}");

    // Hata: move occurs because value has type `X`, which does not implement the `Copy` trait
    /*
    let a @ b = X{a: "gdfgd".into(), b:"dfgdgd".into()};
    println!("{:?}-{:?}",a,b);
    */
}
