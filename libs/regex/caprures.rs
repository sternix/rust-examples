/*

Capturing Groups
grup için () ifadesi gerekiyor

*/

use regex::Regex;

fn main() {
    let re = Regex::new(r"(\w+) (\w+)").unwrap();
    let caps = re.captures("hello world").unwrap();

    assert_eq!("hello", &caps[1]);
    assert_eq!("world", &caps[2]);

    println!("len: {}", caps.len());

    for i in 0..caps.len() {
        println!("{}: {}", i, &caps[i]);
    }
}

/*
ilk capture kelimenin tamamı,

len: 3
0: hello world
1: hello
2: world

*/
