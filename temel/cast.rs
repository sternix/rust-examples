fn main() {
    // rakam dönüşümü
    let dec: f64 = 54.321;
    let int = dec as u16;

    println!("Decimal: {dec}");
    println!("Integer: {int}");

    let ch = 'A';
    let int = ch as u8;
    println!("Char: {ch}");
    println!("Integer: {int}");

    // sadece u8 char'a dönüştürülebiliyor
    let int: u8 = 97;
    let ch = int as char;
    println!("Integer: {int}");
    println!("Char: {ch}");

    let int = 97;
    // burada Option dönüyor
    let ch = char::from_u32(int).unwrap();
    println!("Integer: {int}");
    println!("Char: {ch}");

    let b = true;
    let int = b as i32;
    println!("Bool: {b}");
    println!("Integer: {int}");

    let b = false;
    let int = b as i32;
    println!("Bool: {b}");
    println!("Integer: {int}");

    // int'ten bool'a cast yapılamıyor
}
