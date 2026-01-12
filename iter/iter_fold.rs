/*
fold: bükmek, katlamak

pub fn fold<B, F>(self, init: B, f: F) -> B

fold(ilk_değer, |toplam, item| sonuc)
burada toplama accumulator'de deniliyor

Folds every element into an accumulator by applying an operation, returning the final result.
*/

fn triangle_klasik(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..n + 1 {
        sum += i;
    }
    sum
}

// aynı işlemi yapıyor

fn triangle(n: i32) -> i32 {
    (1..n + 1).fold(0, |sum, item| sum + item)
}

fn main() {
    println!("{}",triangle(3));
    println!("{}",triangle_klasik(3));
}
