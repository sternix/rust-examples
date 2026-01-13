/*

Case Insensitive Match
burada içerisinde rust geçen büyük küçük hepsini
bulabiliyor

(?i)

*/

use regex::Regex;

fn main() {
    let re = Regex::new(r"(?i)rust").unwrap();
    assert!(re.is_match("I love Rust!"));
    assert!(re.is_match("I love rust!"));
    assert!(re.is_match("I love RUST!"));
    assert!(re.is_match("I love rustlang!"));
    assert!(re.is_match("I love RustLang!"));
    assert!(re.is_match("I love RUSTLANG!"));
}
