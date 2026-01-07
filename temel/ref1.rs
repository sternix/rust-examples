/*
    *r = s;
    if r is mutable reference, move or copy s to target memory

    s = *r;
    Make s a copy of whatever r references, if that is Copy
*/

fn main() {
    // initalize reference
    let r: &mut i32 = &mut 0;

    // burada kopyasını alıyor
    let x = 28;
    *r = x;

    assert_eq!(*r, 28);
    assert_eq!(x, 28);

    *r = 128;

    let s = *r;
    assert_eq!(s, 128);
    assert_eq!(*r, 128);

    let r: &mut String = &mut "".to_string();

    let s = "Movable String".to_string();
    *r = s;

    // s, r ye taşındı (moved)
    //  println!("{}",s);
    println!("{}", r);

    /*
       String tipi Copy olmadığından buna izin verilmiyor
    let x = *r;
    */

    let x = Box::new("Movable String 2".to_string());
    let s = *x;
    println!("{}", s);

    // x'in taşıdığı değer taşındığından artık kullanamıyoruz
    // println!("{}", x);
}
