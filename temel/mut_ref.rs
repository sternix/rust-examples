/*
Be sure to note the difference between

let mut ref_x: &i32
and
let ref_x: &mut i32.

The first one represents a mutable reference which can be bound to different values, while the second represents a reference to a mutable value.

ör:

*/

fn main() {
    let i = 10;
    let j = 25;
    let mut r: &i32 = &i;
    // r yi değiştir fakat r'nin gösterdiği değeri değiştirme
    println!("{}", *r);
    r = &j;
    println!("{}", *r);
    // HATA
    // *r = 15;

    let mut x = 10;
    let mut y = 25;

    let mut p: &mut i32 = &mut x;
    println!("{}", *p);
    p = &mut y;
    println!("{}", *p);
    *p = 15;
    println!("{}", *p);
}
