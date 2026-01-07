fn main() {
    let n = read_u32_from_stdin("Please enter row count: ");
    let y = (n * 2) - 1;

    for i in (1..n).rev() {
        for j in 0..y {
            let ch = match (i, j) {
                (_, _) if (i == j || j == ((y - 1) - i)) => "*",
                _ => " ",
            };
            print!("{}", ch);
        }
        println!();
    }
    for _ in 0..y {
        print!("*");
    }

    println!();
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

Please enter row count:10
         *
        * *
       *   *
      *     *
     *       *
    *         *
   *           *
  *             *
 *               *
*******************

*/
