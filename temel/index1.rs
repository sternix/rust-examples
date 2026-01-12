// [] opreratörü ya değeri verir yada panic'ler
// fakat slice üzerinde get methodu Option
// döndürdüğünden bir nebze daha güvenli oluyor.

fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;

    /*
    let first = slice[0];

    burası derlenmiyor
    let last = slice[5];
    */

    let first = slice.get(0);
    let last = slice.get(5);
    println!("first: {:?}", first);
    println!("last: {:?}", last);

    // first: Some(1)
    // last: None

    let maybe_last = slice.get(5);
    let last = if maybe_last.is_some() {
        *maybe_last.unwrap()
    } else {
        -1
    };
    println!("last: {:?}", last);

    // yada
    let last = if let Some(l) = slice.get(5) { *l } else { -1 };
    println!("last: {:?}", last);

    // yada

    let last = *slice.get(5).unwrap_or(&-1);
    println!("last: {:?}", last);
}
