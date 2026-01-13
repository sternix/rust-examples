/*

Simple Match
basit karşılaştırma
eğer içinde rust geçiyorsa doğru geçmiyorsa yanlış
case sensitive yani içinde Rust geçeni bulamıyor

*/

use regex::Regex;

fn main() {
    let re = Regex::new(r"rust").unwrap();
    assert!(re.is_match("I love rust!"));

    // bunu bulamıyor
    assert!(!re.is_match("I love Rust!"));
}
