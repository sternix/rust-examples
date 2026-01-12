fn main() {
    let n = read_u32_from_stdin("Please enter row count: ");

    for i in 1..=n {
        for _ in 0..(n - i) {
            print!(" ");
        }

        for s in (1..=i).rev() {
            print!("{}", s);
        }

        for s in 2..=i {
            print!("{}", s);
        }

        println!();
    }

    for i in (1..n).rev() {
        for _ in 0..(n - i) {
            print!(" ");
        }

        for s in (1..=i).rev() {
            print!("{}", s);
        }

        for s in 2..=i {
            print!("{}", s);
        }

        println!();
    }
}

use std::io::{Write, stdin, stdout};

fn read_u32_from_stdin(q: &str) -> u32 {
    print!("{}", q);
    stdout().flush().unwrap();

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    s.trim().parse().unwrap()
}
/*
        1
       212
      32123
     4321234
    543212345
   65432123456
  7654321234567
 876543212345678
98765432123456789
 876543212345678
  7654321234567
   65432123456
    543212345
     4321234
      32123
       212
        1
*/
