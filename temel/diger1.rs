fn main() {
    println!("{}", 0b101101u8.count_ones()); // 1 olan bitlerin sayısı
    println!("{}", 2i8.pow(4)); // 2 üzeri 4
    // pow metodunda tipin bildirilmesi yada inferred edilmiş olması gerekiyor.
    // 2.pow(4) hata veriyor
}
