fn main() {
    let mut s = String::new();
    s.push('H');
    s.push_str("ello");
    s.push(' ');
    s += "World!";
    s.pop(); // son ünlemi çıkar
    assert_eq!(s, "Hello World");
}
