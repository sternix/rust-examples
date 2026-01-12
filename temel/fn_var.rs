fn print(s: &str) {
    println!("{}", s);
}

fn len(s: &str) -> usize {
    s.len()
}

fn main() {
    let f = print;
    f("testststs");

    // tip ile birlikte
    let f: fn(&str) = print;
    f("dsdgdfgd");

    let f = len;
    println!("{}", f("12345"));

    let f: fn(&str) -> usize = len;
    println!("{}", f("12345"));
}
