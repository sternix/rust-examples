struct X {
    x: i32,
}

fn main() {
    let f = X { x: 28 };
    let r: &X = &f;
    let rr: &&X = &r;
    let rrr: &&&X = &rr;

    assert_eq!(rrr.x, 28);
}
