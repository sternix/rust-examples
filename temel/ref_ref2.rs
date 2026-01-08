fn main() {
    let x = 10;
    let y = 10;
    let rx = &x;
    let ry = &y;
    let rrx = &rx;
    let rry = &ry;

    println!("{}", rrx); // 10
    println!("{}", rry); // 10

    assert!(rrx == rry);
}
