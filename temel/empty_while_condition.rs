fn main() {
    let mut x = 0;

    // aslında boş değil {} içinden bool bir değer dönmesi gerekiyor
    // rust'ta {} içerisi bir expression oluyor

    // do  {} while gibi
    while {
        x += 1;
        println!("{x}");
        x <= 10
    } {}
}
