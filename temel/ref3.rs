fn main() {
    let x = 10;
    let y = 20;

    let mut r = &x;
    println!("{}", *r); // 10
    r = &y;
    println!("{}", *r); // 20
}
