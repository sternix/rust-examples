use regex::Regex;

fn main() {
    let unicode_re = Regex::new(r"ça").unwrap();
    assert!(unicode_re.is_match("ça va?"));

    let byte_re = Regex::new(r"(?-u)ça").unwrap();
    assert!(!byte_re.is_match("ça va?")); // Fails on byte string
}
