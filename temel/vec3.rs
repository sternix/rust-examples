fn main() {
    /*
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    aşağıdaki vec! makrosu ile aynı işi yapıyor,
    eğer makro kullanır ve yeni değer eklememize gerek yok ise
    mut olarak tanımlamamıza gerek kalmıyor.
    */

    let v = vec![10, 20, 30];

    let first = v[0]; // eğer v[5] yapsak panic'liyor
    let maybe_first = v.get(0); // fakat bu Some yada None döndürüyor

    println!("first {:?}", first);
    println!("maybe_first {:?}", maybe_first);

    // first 10
    // maybe_first Some(10)
}
