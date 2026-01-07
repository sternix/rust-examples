fn main() {
    assert_eq!('*' as i32, 42);
    assert_eq!('A' as i32, 65);
    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('8'.to_digit(10), Some(8));
    assert_eq!('x'.len_utf8(), 1);
    assert_eq!(std::char::from_digit(2, 10), Some('2'));
}
