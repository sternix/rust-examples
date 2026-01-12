fn main() {
    let a = [1, 2, 3];

    let mut it = a.iter().map(|e| std::char::from_u32(e + 64).unwrap());

    /*
       Burada A: 65 olduğundan ne verilirse 64 ekleniyor
       yani 1 - A, 2 - B, 3 - C şeklinde
       verilen rakamlardan karakter iterator'ü elde edildi
    */

    assert_eq!(it.next(), Some('A'));
    assert_eq!(it.next(), Some('B'));
    assert_eq!(it.next(), Some('C'));
}
