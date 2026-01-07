use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin()
        .read_line(&mut s)
        .expect("stdin'den okunurken bir hata oluştu");
    let i: i32 = s.trim().parse().expect("int tipinde bir değer giriniz!");
    // önemli olan burada i'ye type vermek
    println!("you typed '{}'", i);
}
