fn main() {
    let hi: u8 = 0b11100000; // 0xe0;
    println!("{:b}", hi);
    let lo = !hi; // 0x1f
    println!("{:b}", lo);
    // 00011111 -> 11111
}
