// https://stackoverflow.com/questions/70196449/with-rust-regular-expressions-how-can-i-use-named-capture-groups-preceding-a-st

use regex::Regex;

fn main() {
    let re = Regex::new(r"(?P<n>b)").unwrap();
    let before = "abc";
    assert_eq!(re.replace_all(before, "$nB"), "ac");
    assert_eq!(re.replace_all(before, "${n}B"), "abBc");
}
