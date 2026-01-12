/*

== operator follows all the references and performs the comparison on their final targets x and y
if you actually want to know whether two references point to the same memory, you can use std::ptr::eq which compares
them as addresses

*/

fn main() {
    let x = 10;
    let y = 10;
    let rx = &x;
    let _rx = rx;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;

    println!("{}", rrx); // 10
    println!("{}", rry); // 10

    // bu referansı takip edip
    // referansların gösterdikleri değerleri kıyaslıyor
    // ikiside 10 olduğundan sonuç true
    assert!(rrx == rry);

    // iki referans'ın adresler değerlerini kıyaslıyor
    // ikiside farklı adres olduğundan sonuç false
    assert!(!std::ptr::eq(rrx, rry));

    // şimdi pointer adresleri eşit
    assert!(std::ptr::eq(rx, _rx));
}
