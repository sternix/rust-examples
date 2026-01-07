// hem &str hemde String çağrılabiliyor
fn dump(s: &str) {
    println!("str '{}'", s);
}

fn main() {
    let text = "Hello World!";
    let s = text.to_string();
    dump(text);
    dump(&s);
}
