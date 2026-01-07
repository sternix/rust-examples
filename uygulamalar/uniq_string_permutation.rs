// https://www.geeksforgeeks.org/print-all-permutations-of-a-string-in-java/

use std::char;
use std::env;

fn print_distinct_permutn(s: &str, ans: &str) {
    if s.is_empty() {
        println!("{} ", ans);
        return;
    }

    let mut alpha = [false; 26];
    let a = 'a' as usize;

    for i in 0..s.len() {
        let ch = s.chars().nth(i).unwrap() as usize;
        let ros = format!("{}{}", &s[0..i], &s[(i + 1)..]);

        if !alpha[ch - a] {
            let c = char::from_u32(ch as u32).unwrap();
            let ans = format!("{}{}", &ans, c);
            print_distinct_permutn(&ros, &ans);
        }

        alpha[ch - a] = true;
    }
}

fn main() {
    let s = env::args().nth(1).expect("lütfen bir kelime girin");
    print_distinct_permutn(&s, "");
}
