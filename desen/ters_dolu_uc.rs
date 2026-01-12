fn main() {
    let n = read_u32_from_stdin("Please enter row count: ");

    for i in (1..n).rev() {
        for _ in 0..i {
            print!(" ");
        }
        for _ in 0..(n - i) {
            print!("*");
        }
        println!();
    }

    for i in 0..n {
        for _ in 0..i {
            print!(" ");
        }
        for _ in 0..(n - i) {
            print!("*");
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

Please enter row count: 7
      *
     **
    ***
   ****
  *****
 ******
*******
 ******
  *****
   ****
    ***
     **
      *

*/
