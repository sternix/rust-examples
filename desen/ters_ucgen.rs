fn main() {
    let n = read_u32_from_stdin("Enter the number of rows: ");
    let y = (n * 2) - 1;

    for _ in 0..y {
        print!("*");
    }
    println!();

    for i in 1..n {
        for j in 0..y {
            let ch = if i == j {
                "*"
            } else if j == ((y - 1) - i) {
                "*"
            } else {
                " "
            };

            print!("{}", ch);
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
Enter the number of rows: 10
*******************
 *               *
  *             *
   *           *
    *         *
     *       *
      *     *
       *   *
        * *
         *
*/
