/*
Burada #[derive(Copy, Clone)] satırı olmazsa hata veriyor
eğer struct'ın tüm alanları Copy ise #derive Copy olabilir yoksa hata veriyor
this field does not implement Copy
*/

#[derive(Copy, Clone)]
struct Label {
    number: u32,
}

fn print(l: Label) {
    println!("STAMP: {}", l.number);
}

fn main() {
    let l = Label { number: 3 };
    print(l);
    // eğer derive satırını kaldırsak
    // ve print(&l); ve print fonksiyonunuda l: &Label yapsak çalışıyor
    println!("My label number is: {}", l.number);
}
