use std::io::{Write, stdout};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let s: i32 = args[1]
            .parse()
            .expect("Lütfen int tipinde bir parametre giriniz!");

        for i in 1..=s {
            // boşluklar
            for _ in 1..=(s - i) {
                print!(" ");
            }

            // sol taraf
            for _ in 1..=i {
                print!("*");
            }

            // sağ taraf
            for _ in 0..(i - 1) {
                print!("*");
            }

            println!();
        }

        for i in (1..s).rev() {
            // boşluk
            for _ in 1..=(s - i) {
                print!(" ");
            }

            // sağ taraf
            for _ in 1..=i {
                print!("*");
            }

            // sol taraf
            for _ in 0..(i - 1) {
                print!("*");
            }

            println!();
        }

        stdout().flush().expect("Hata");
    }
}

/*
 *
 ***
 *****
 *******
 *********
 *******
 *****
 ***
 *
*/
