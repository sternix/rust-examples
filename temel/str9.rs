fn main() {
    let s = "üğişçöıIĞÜİŞÇÖ";

    println!("{} - {}", s.len(), s.chars().count());
    // 26 - 14

    for c in s.chars() {
        print!("{} ", c);
    }
    println!("");
}
