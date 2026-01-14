use std::io::{Write, stdin, stdout};

fn main() {
    print!("Enter the number of rows: ");
    stdout().flush().unwrap();

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let n: u32 = s.trim().parse().unwrap();

    for i in 1..=n {
        for _ in i..n {
            print!(" ");
        }

        for k in 1..(i * 2) {
            print!("{}", k);
        }

        println!();
    }
}

/*

Enter the number of rows: 5
    1
   123
  12345
 1234567
123456789

*/
