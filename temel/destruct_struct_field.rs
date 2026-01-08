struct X {
    f1: i32,
    f2: i32,
    f3: i32,
    f4: i32,
}

fn main() {
    let x = X {
        f1: 10,
        f2: 20,
        f3: 30,
        f4: 40,
    };
    // sadece ilgilendiğimizi destruct edebiliriz
    let X { f3, .. } = x;
    println!("{}", f3);
}
