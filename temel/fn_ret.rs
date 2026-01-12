fn fn1() {
    println!("fn1");
}

fn fn2() -> () {
    println!("fn2");
}

fn main() {
    let r = fn1();
    if r == () {
        println!("Return value is Unit Type");
    }

    let r = fn2();
    if r == () {
        println!("Return value is Unit Type");
    }
}
