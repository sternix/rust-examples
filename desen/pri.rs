use std::io::{Write, stdout};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let s: i32 = args[1]
            .parse()
            .expect("Lütfen int tipinde bir parametre giriniz!");

        for i in 1..=s {
            for _ in 1..=(s - i) {
                print!(" ");
            }

            for n in 1..=i {
                print!("{}", n);
            }

            for n in (1..=(i - 1)).rev() {
                print!("{}", n);
            }

            println!();
        }

        stdout().flush().expect("Hata");
    }
}

/*
    1
   121
  12321
 1234321
123454321
*/

/*
#include <stdio.h>

int main() {
    printf("Sayı: \n");
    int n;
    scanf("%d",&n);

    for (int i = 1; i <= n; i++) {
        for (int j = 1; j <= (n - i); j++) {
            printf(" ");
        }

        for (int num = 1; num <= i; num++) {
            printf("%d",num);
        }

        for (int num = (i - 1); num >= 1; num--) {
            printf("%d",num);
        }

        printf("\n");
    }
}
*/
