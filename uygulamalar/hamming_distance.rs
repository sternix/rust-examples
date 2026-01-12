use std::iter::zip;

fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        None
    } else {
        let distance = zip(s1.chars(), s2.chars())
            .filter(|(s1, s2)| s1 != s2)
            .count();

        Some(distance)
    }
}

fn main() {
    assert_eq!(hamming_distance("GGACGGATTCTG", "AGGACGGATTCT"), Some(9));
    assert_eq!(hamming_distance("AATG", "AAA"), None);
    assert_eq!(hamming_distance("GGACTGAAATCTG", "GGACTGAAATCTG"), Some(0));
}
