// Regex Set

use regex::RegexSet;

fn main() {
    let set = RegexSet::new(&[r"rust", r"java", r"python"]).unwrap();
    let matches = set.matches("I love rust and python");
    assert!(matches.matched(0));
    assert!(!matches.matched(1)); // <- java olmadığından bu false !!!
    assert!(matches.matched(2));
}
