fn main() {
    let n = read_u32_from_stdin("Please enter row count: ");
    let y = n * 2;

    for i in 1..=n {
        for j in 1..=y - 1 {
            let c = if i == j {
                i.to_string()
            } else if j == (y - i) {
                i.to_string()
            } else {
                " ".to_string()
            };

            print!("{}", c);
        }
        println!();
    }

    for i in (1..n).rev() {
        for j in 1..=y - 1 {
            let c = if i == j {
                i.to_string()
            } else if j == (y - i) {
                i.to_string()
            } else {
                " ".to_string()
            };

            print!("{}", c);
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
1           1
 2         2
  3       3
   4     4
    5   5
     6 6
      7
     6 6
    5   5
   4     4
  3       3
 2         2
1           1

*/
