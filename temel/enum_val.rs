/*
enum değerleri varsayılan bir değer verilmezse 0'dan başlar.
varyasyonların içinde en yüksek alanı kaplayan enum'un hafıza kullanımını belirler.
*/

enum En {
    X,
    Y,
    Z = 256, // 255'e kadar 1 byte kaplıyor, 256'da 2 byte oluyor
}

enum En2 {
    _X,
    _Y,
    _Z = 255, // 255'e kadar 1 byte kaplıyor
}

fn main() {
    println!("{}", En::X as i32);
    println!("{}", En::Y as i32);
    println!("{}", En::Z as i32);

    assert_eq!(std::mem::size_of::<En>(), 2);
    assert_eq!(std::mem::size_of::<En2>(), 1);
}
