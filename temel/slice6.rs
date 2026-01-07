fn print(s: &[f64]) {
    for i in s {
        print!("{} ", i);
    }
    println!("");
}

fn main() {
    let v = vec![0.0, 0.707, 1.0, 0.707];
    let a = [0.0, 0.707, 1.0, 0.707];

    let sv = &v; // tipi &[f64]
    let sa = &a; // tipi &[f64]

    assert_eq!(sv, sa);

    println!("{:?}", sv);
    println!("{:?}", sa);

    print(&v);
    print(&a);
}
