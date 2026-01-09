// bit short-circuit

fn a() -> bool {
    println!("a");
    true
}

fn b() -> bool {
    println!("b");
    true
}

fn main() {
    if a() | b() {
        println!("a | b");
    }

    if a() || b() {
        println!("a | b");
    }
}
