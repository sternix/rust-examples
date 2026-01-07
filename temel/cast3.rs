// Tip dönüşümleri

fn main() {
    assert_eq!(1000_i16 as u8, 232_u8);
    assert_eq!(65535_u32 as i16, -1_i16);
    assert_eq!(-1_i8 as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);
    assert_eq!(std::u64::MAX as i8, -1_i8);
}
