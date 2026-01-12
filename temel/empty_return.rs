fn f() {
    return;
}

fn f2() -> () {
    ()
}

fn main() {
    if f() == () {
        println!("Empty Return");
    }

    if f2() == () {
        println!("Empty Return");
    }
}
